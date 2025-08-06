//! # Crowdfunding Smart Contract
//!
//! A comprehensive crowdfunding platform demonstrating real-world DeFi functionality:
//! - Goal-based funding with time limits
//! - Refund mechanisms if goals aren't met
//! - Milestone-based fund release
//! - Contributor tracking and rewards
//! - Administrative controls and emergency mechanisms
//!
//! This contract showcases advanced patterns for decentralized fundraising.

#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

declare_id!("Crowdfunding1111111111111111111111111111112");

/// Campaign status enumeration
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CampaignStatus {
    Active = 0,
    Successful = 1,
    Failed = 2,
    Cancelled = 3,
}

impl CampaignStatus {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => CampaignStatus::Successful,
            2 => CampaignStatus::Failed,
            3 => CampaignStatus::Cancelled,
            _ => CampaignStatus::Active,
        }
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Campaign data structure
#[derive(Clone, Debug)]
pub struct Campaign {
    pub id: u32,
    pub creator: H160,
    pub title: ByteString,
    pub description: ByteString,
    pub goal_amount: Int256,
    pub deadline: u64,
    pub total_raised: Int256,
    pub status: CampaignStatus,
    pub milestone_count: u32,
    pub released_amount: Int256,
}

/// Contribution data structure
#[derive(Clone, Debug)]
pub struct Contribution {
    pub contributor: H160,
    pub campaign_id: u32,
    pub amount: Int256,
    pub timestamp: u64,
}

/// Platform data structure
#[derive(Clone, Debug)]
pub struct PlatformData {
    pub owner: H160,
    pub platform_fee_bp: u32, // basis points (100 = 1%)
    pub campaign_count: u32,
    pub is_paused: bool,
}

/// Initialize context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(init)]
    pub platform: AccountInfo<'info>,
}

/// Create campaign context
#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(signer)]
    pub creator: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
    #[account(init)]
    pub campaign: AccountInfo<'info>,
}

/// Contribute context
#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(signer)]
    pub contributor: AccountInfo<'info>,
    #[account(mut)]
    pub campaign: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
}

/// Withdraw context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(signer)]
    pub creator: AccountInfo<'info>,
    #[account(mut)]
    pub campaign: AccountInfo<'info>,
}

/// Refund context
#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(signer)]
    pub contributor: AccountInfo<'info>,
    #[account(mut)]
    pub campaign: AccountInfo<'info>,
}

/// Admin context
#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub platform: AccountInfo<'info>,
}

/// View context
#[derive(Accounts)]
pub struct View<'info> {
    pub account: AccountInfo<'info>,
}

#[program]
pub mod crowdfunding {
    use super::*;

    /// Initialize the platform (one-time setup)
    pub fn initialize(
        ctx: Context<Initialize>,
        platform_fee_bp: u32
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let platform = &mut ctx.accounts.platform;

        require!(platform_fee_bp <= 1000, "Platform fee too high (max 10%)");

        // Initialize platform data
        let platform_data = PlatformData {
            owner: owner.key(),
            platform_fee_bp,
            campaign_count: 0,
            is_paused: false,
        };

        // Store platform data
        let storage = Storage::get_context();
        Storage::put(storage, ByteString::from_literal("platform_data"), platform_data.serialize());

        emit!(PlatformInitialized { 
            owner: owner.key(), 
            platform_fee_bp: Int256::new(platform_fee_bp as i64),
        });
        Ok(())
    }

    /// Create a new crowdfunding campaign
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        title: ByteString,
        description: ByteString,
        goal_amount: Int256,
        duration_days: u32,
        milestone_count: u32
    ) -> Result<()> {
        let creator = &ctx.accounts.creator;
        let platform = &mut ctx.accounts.platform;
        let campaign = &mut ctx.accounts.campaign;

        // Validate parameters
        require!(!title.is_empty() && title.len() <= 100, "Invalid title length");
        require!(!description.is_empty() && description.len() <= 1000, "Invalid description length");
        require!(goal_amount > Int256::zero(), "Goal amount must be positive");
        require!(duration_days > 0 && duration_days <= 365, "Invalid duration (1-365 days)");
        require!(milestone_count > 0 && milestone_count <= 10, "Invalid milestone count (1-10)");

        let storage = Storage::get_context();
        let mut platform_data = get_platform_data(&storage)?;
        
        require!(!platform_data.is_paused, "Platform is paused");

        // Create campaign
        let current_time = Runtime::get_time();
        let deadline = current_time + (duration_days as u64 * 24 * 60 * 60 * 1000); // milliseconds

        platform_data.campaign_count += 1;
        let campaign_id = platform_data.campaign_count;

        let campaign_data = Campaign {
            id: campaign_id,
            creator: creator.key(),
            title: title.clone(),
            description: description.clone(),
            goal_amount,
            deadline,
            total_raised: Int256::zero(),
            status: CampaignStatus::Active,
            milestone_count,
            released_amount: Int256::zero(),
        };

        // Store campaign data
        let campaign_key = format!("campaign_{}", campaign_id);
        Storage::put(storage.clone(), ByteString::from_str(&campaign_key), campaign_data.serialize());

        // Update platform data
        Storage::put(storage, ByteString::from_literal("platform_data"), platform_data.serialize());

        emit!(CampaignCreated {
            campaign_id,
            creator: creator.key(),
            title,
            goal_amount,
            deadline,
        });
        Ok(())
    }

    /// Contribute to a campaign
    pub fn contribute(
        ctx: Context<Contribute>,
        campaign_id: u32,
        amount: Int256
    ) -> Result<()> {
        let contributor = &ctx.accounts.contributor;
        let campaign = &mut ctx.accounts.campaign;
        let platform = &mut ctx.accounts.platform;

        require!(amount > Int256::zero(), "Contribution must be positive");

        let storage = Storage::get_context();
        let platform_data = get_platform_data(&storage)?;
        require!(!platform_data.is_paused, "Platform is paused");

        // Get campaign data
        let campaign_key = format!("campaign_{}", campaign_id);
        let mut campaign_data: Campaign = match Storage::get(storage.clone(), ByteString::from_str(&campaign_key)) {
            Some(data) => Campaign::deserialize(data)?,
            None => return Err(ErrorCode::CampaignNotFound.into()),
        };

        require!(campaign_data.status == CampaignStatus::Active, "Campaign not active");
        require!(Runtime::get_time() < campaign_data.deadline, "Campaign deadline passed");

        // Update campaign total
        campaign_data.total_raised = campaign_data.total_raised.checked_add(&amount);

        // Check if goal reached
        if campaign_data.total_raised >= campaign_data.goal_amount {
            campaign_data.status = CampaignStatus::Successful;
        }

        // Store updated campaign
        Storage::put(storage.clone(), ByteString::from_str(&campaign_key), campaign_data.serialize());

        // Track individual contribution
        let contribution_key = format!("contribution_{}_{}", campaign_id, contributor.key());
        let existing_contribution = match Storage::get(storage.clone(), ByteString::from_str(&contribution_key)) {
            Some(data) => Int256::from_byte_string(data),
            None => Int256::zero(),
        };
        let new_contribution = existing_contribution.checked_add(&amount);
        Storage::put(storage.clone(), ByteString::from_str(&contribution_key), new_contribution.into_byte_string());

        // Add to contributors list (simplified)
        let contributors_key = format!("contributors_{}", campaign_id);
        let mut contributors = match Storage::get(storage.clone(), ByteString::from_str(&contributors_key)) {
            Some(data) => deserialize_contributor_list(data),
            None => Array::new(),
        };

        // Add contributor if not already in list
        let mut found = false;
        for i in 0..contributors.size() {
            if contributors.get(i) == contributor.key() {
                found = true;
                break;
            }
        }
        if !found {
            contributors.push(contributor.key());
        }

        Storage::put(storage, ByteString::from_str(&contributors_key), serialize_contributor_list(&contributors));

        emit!(ContributionMade {
            campaign_id,
            contributor: contributor.key(),
            amount,
            total_raised: campaign_data.total_raised,
        });

        if campaign_data.status == CampaignStatus::Successful {
            emit!(CampaignSuccessful { campaign_id, total_raised: campaign_data.total_raised });
        }

        Ok(())
    }

    /// Withdraw funds (creator only for successful campaigns)
    pub fn withdraw_funds(
        ctx: Context<Withdraw>,
        campaign_id: u32,
        milestone_id: u32
    ) -> Result<()> {
        let creator = &ctx.accounts.creator;
        let campaign = &mut ctx.accounts.campaign;

        let storage = Storage::get_context();

        // Get campaign data
        let campaign_key = format!("campaign_{}", campaign_id);
        let mut campaign_data: Campaign = match Storage::get(storage.clone(), ByteString::from_str(&campaign_key)) {
            Some(data) => Campaign::deserialize(data)?,
            None => return Err(ErrorCode::CampaignNotFound.into()),
        };

        require!(campaign_data.creator == creator.key(), "Unauthorized: Not campaign creator");
        require!(campaign_data.status == CampaignStatus::Successful, "Campaign not successful");
        require!(milestone_id > 0 && milestone_id <= campaign_data.milestone_count, "Invalid milestone ID");

        // Check if milestone already released
        let milestone_key = format!("milestone_{}_{}", campaign_id, milestone_id);
        require!(
            Storage::get(storage.clone(), ByteString::from_str(&milestone_key)).is_none(),
            "Milestone already released"
        );

        // Calculate milestone amount (equal distribution)
        let milestone_amount = campaign_data.total_raised.checked_div(&Int256::new(campaign_data.milestone_count as i64));
        
        // Calculate platform fee
        let platform_data = get_platform_data(&storage)?;
        let fee_amount = milestone_amount.checked_mul(&Int256::new(platform_data.platform_fee_bp as i64))
            .checked_div(&Int256::new(10000));
        let creator_amount = milestone_amount.checked_sub(&fee_amount);

        // Mark milestone as released
        Storage::put(storage.clone(), ByteString::from_str(&milestone_key), ByteString::from_literal("released"));

        // Update released amount
        campaign_data.released_amount = campaign_data.released_amount.checked_add(&milestone_amount);
        Storage::put(storage, ByteString::from_str(&campaign_key), campaign_data.serialize());

        emit!(FundsWithdrawn {
            campaign_id,
            milestone_id,
            creator: creator.key(),
            amount: creator_amount,
            platform_fee: fee_amount,
        });

        Ok(())
    }

    /// Request refund (contributor only for failed campaigns)
    pub fn request_refund(
        ctx: Context<Refund>,
        campaign_id: u32
    ) -> Result<()> {
        let contributor = &ctx.accounts.contributor;
        let campaign = &mut ctx.accounts.campaign;

        let storage = Storage::get_context();

        // Get campaign data
        let campaign_key = format!("campaign_{}", campaign_id);
        let mut campaign_data: Campaign = match Storage::get(storage.clone(), ByteString::from_str(&campaign_key)) {
            Some(data) => Campaign::deserialize(data)?,
            None => return Err(ErrorCode::CampaignNotFound.into()),
        };

        // Check if campaign failed (deadline passed and goal not met)
        if campaign_data.status == CampaignStatus::Active && Runtime::get_time() >= campaign_data.deadline {
            if campaign_data.total_raised < campaign_data.goal_amount {
                campaign_data.status = CampaignStatus::Failed;
                Storage::put(storage.clone(), ByteString::from_str(&campaign_key), campaign_data.serialize());
            }
        }

        require!(campaign_data.status == CampaignStatus::Failed, "Campaign not failed");

        // Get contributor's contribution
        let contribution_key = format!("contribution_{}_{}", campaign_id, contributor.key());
        let contribution_amount = match Storage::get(storage.clone(), ByteString::from_str(&contribution_key)) {
            Some(data) => Int256::from_byte_string(data),
            None => return Err(ErrorCode::NoContribution.into()),
        };

        require!(contribution_amount > Int256::zero(), "No contribution found");

        // Mark as refunded
        Storage::delete(storage, ByteString::from_str(&contribution_key));

        emit!(RefundProcessed {
            campaign_id,
            contributor: contributor.key(),
            amount: contribution_amount,
        });

        Ok(())
    }

    /// Cancel campaign (creator only, before deadline)
    pub fn cancel_campaign(
        ctx: Context<Withdraw>,
        campaign_id: u32
    ) -> Result<()> {
        let creator = &ctx.accounts.creator;
        let campaign = &mut ctx.accounts.campaign;

        let storage = Storage::get_context();

        // Get campaign data
        let campaign_key = format!("campaign_{}", campaign_id);
        let mut campaign_data: Campaign = match Storage::get(storage.clone(), ByteString::from_str(&campaign_key)) {
            Some(data) => Campaign::deserialize(data)?,
            None => return Err(ErrorCode::CampaignNotFound.into()),
        };

        require!(campaign_data.creator == creator.key(), "Unauthorized: Not campaign creator");
        require!(campaign_data.status == CampaignStatus::Active, "Campaign not active");
        require!(campaign_data.released_amount == Int256::zero(), "Funds already released");

        // Cancel campaign
        campaign_data.status = CampaignStatus::Cancelled;
        Storage::put(storage, ByteString::from_str(&campaign_key), campaign_data.serialize());

        emit!(CampaignCancelled { campaign_id });
        Ok(())
    }

    /// Emergency pause (owner only)
    pub fn emergency_pause(ctx: Context<AdminAction>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let platform = &mut ctx.accounts.platform;

        let storage = Storage::get_context();
        let mut platform_data = get_platform_data(&storage)?;

        require!(owner.key() == platform_data.owner, "Unauthorized: Not platform owner");

        platform_data.is_paused = true;
        Storage::put(storage, ByteString::from_literal("platform_data"), platform_data.serialize());

        emit!(PlatformPaused {});
        Ok(())
    }

    /// Get campaign info
    pub fn get_campaign(ctx: Context<View>, campaign_id: u32) -> Result<Campaign> {
        let storage = Storage::get_context();
        let campaign_key = format!("campaign_{}", campaign_id);
        
        match Storage::get(storage, ByteString::from_str(&campaign_key)) {
            Some(data) => Campaign::deserialize(data),
            None => Err(ErrorCode::CampaignNotFound.into()),
        }
    }

    /// Get platform stats
    pub fn get_platform_stats(ctx: Context<View>) -> Result<PlatformData> {
        let storage = Storage::get_context();
        get_platform_data(&storage)
    }
}

// Helper functions
fn get_platform_data(storage: &Storage) -> Result<PlatformData> {
    match Storage::get(storage.clone(), ByteString::from_literal("platform_data")) {
        Some(data) => PlatformData::deserialize(data),
        None => Err(ErrorCode::PlatformNotInitialized.into()),
    }
}

fn serialize_contributor_list(contributors: &Array<H160>) -> ByteString {
    let mut result = ByteString::empty();
    let len = contributors.size() as u32;
    result = result.concat(&ByteString::from_bytes(&len.to_le_bytes()));
    
    for i in 0..contributors.size() {
        let contributor = contributors.get(i);
        result = result.concat(&contributor.into_byte_string());
    }
    
    result
}

fn deserialize_contributor_list(data: ByteString) -> Array<H160> {
    let bytes = data.to_bytes();
    let mut contributors = Array::new();
    
    if bytes.len() < 4 {
        return contributors;
    }
    
    let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    let mut offset = 4;
    
    for _ in 0..len {
        if offset + 20 <= bytes.len() {
            let contributor = H160::from_byte_string(ByteString::from_bytes(&bytes[offset..offset + 20]));
            contributors.push(contributor);
            offset += 20;
        }
    }
    
    contributors
}

// Events
#[event]
pub struct PlatformInitialized {
    pub owner: H160,
    pub platform_fee_bp: Int256,
}

#[event]
pub struct CampaignCreated {
    pub campaign_id: u32,
    pub creator: H160,
    pub title: ByteString,
    pub goal_amount: Int256,
    pub deadline: u64,
}

#[event]
pub struct ContributionMade {
    pub campaign_id: u32,
    pub contributor: H160,
    pub amount: Int256,
    pub total_raised: Int256,
}

#[event]
pub struct CampaignSuccessful {
    pub campaign_id: u32,
    pub total_raised: Int256,
}

#[event]
pub struct FundsWithdrawn {
    pub campaign_id: u32,
    pub milestone_id: u32,
    pub creator: H160,
    pub amount: Int256,
    pub platform_fee: Int256,
}

#[event]
pub struct RefundProcessed {
    pub campaign_id: u32,
    pub contributor: H160,
    pub amount: Int256,
}

#[event]
pub struct CampaignCancelled {
    pub campaign_id: u32,
}

#[event]
pub struct PlatformPaused {}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Platform not initialized")]
    PlatformNotInitialized,
    #[msg("Campaign not found")]
    CampaignNotFound,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Campaign not active")]
    CampaignNotActive,
    #[msg("Campaign deadline passed")]
    DeadlinePassed,
    #[msg("Goal not reached")]
    GoalNotReached,
    #[msg("No contribution found")]
    NoContribution,
    #[msg("Platform paused")]
    PlatformPaused,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Milestone already released")]
    MilestoneAlreadyReleased,
}