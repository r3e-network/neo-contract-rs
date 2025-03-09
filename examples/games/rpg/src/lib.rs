//! # On-Chain RPG Game
//!
//! A simple on-chain RPG game with character management, inventory,
//! quests, and combat mechanics. This example demonstrates how complex
//! game logic can be implemented in a blockchain environment.

#[neo_contract::contract]
mod neo_rpg {
    use neo_contract::prelude::*;
    
    /// Character stats
    #[derive(Debug, Clone, Encode, Decode)]
    struct Stats {
        strength: u16,
        dexterity: u16,
        intelligence: u16,
        vitality: u16,
        luck: u16,
    }
    
    /// Character class
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum Class {
        Warrior,
        Rogue,
        Mage,
        Ranger,
        Cleric,
    }
    
    /// Item type
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum ItemType {
        Weapon,
        Armor,
        Accessory,
        Consumable,
        QuestItem,
    }
    
    /// Item rarity
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum Rarity {
        Common,
        Uncommon,
        Rare,
        Epic,
        Legendary,
    }
    
    /// Item definition
    #[derive(Debug, Clone, Encode, Decode)]
    struct Item {
        id: u32,
        name: String,
        item_type: ItemType,
        rarity: Rarity,
        level_req: u16,
        class_req: Option<Class>,
        stats: Option<Stats>,
        value: u64,
    }
    
    /// Player's inventory item
    #[derive(Debug, Clone, Encode, Decode)]
    struct InventoryItem {
        item_id: u32,
        quantity: u32,
        equipped: bool,
    }
    
    /// Character representation
    #[derive(Debug, Clone, Encode, Decode)]
    struct Character {
        name: String,
        class: Class,
        level: u16,
        experience: u64,
        gold: u64,
        stats: Stats,
        health: u32,
        max_health: u32,
        mana: u32,
        max_mana: u32,
        inventory: Vec<InventoryItem>,
        equipped_weapon: Option<u32>,
        equipped_armor: Option<u32>,
        equipped_accessory: Option<u32>,
        completed_quests: Vec<u32>,
        active_quests: Vec<u32>,
        created_at: u64,
        last_action: u64,
    }
    
    /// Quest status
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum QuestStatus {
        NotStarted,
        InProgress,
        Completed,
    }
    
    /// Quest objective type
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum ObjectiveType {
        KillMonsters,
        CollectItems,
        CompleteQuest,
    }
    
    /// Quest objective
    #[derive(Debug, Clone, Encode, Decode)]
    struct QuestObjective {
        objective_type: ObjectiveType,
        target_id: u32,
        amount: u32,
    }
    
    /// Quest definition
    #[derive(Debug, Clone, Encode, Decode)]
    struct Quest {
        id: u32,
        name: String,
        description: String,
        level_req: u16,
        prerequisite_quests: Vec<u32>,
        objectives: Vec<QuestObjective>,
        reward_experience: u64,
        reward_gold: u64,
        reward_items: Vec<(u32, u32)>, // (item_id, quantity)
    }
    
    /// Monster definition
    #[derive(Debug, Clone, Encode, Decode)]
    struct Monster {
        id: u32,
        name: String,
        level: u16,
        health: u32,
        attack: u16,
        defense: u16,
        experience_reward: u64,
        gold_reward: u64,
        item_drops: Vec<(u32, u8)>, // (item_id, drop_chance_percentage)
    }
    
    /// Combat log entry
    #[derive(Debug, Clone, Encode, Decode)]
    struct CombatLogEntry {
        attacker: String,
        defender: String,
        damage: u32,
        critical: bool,
        timestamp: u64,
    }
    
    /// Combat session
    #[derive(Debug, Clone, Encode, Decode)]
    struct CombatSession {
        character_id: Address,
        monster_id: u32,
        monster_current_health: u32,
        combat_log: Vec<CombatLogEntry>,
        started_at: u64,
        status: CombatStatus,
    }
    
    /// Combat status
    #[derive(Debug, Clone, Encode, Decode, PartialEq)]
    enum CombatStatus {
        InProgress,
        Victory,
        Defeat,
    }
    
    /// Player progress on a quest
    #[derive(Debug, Clone, Encode, Decode)]
    struct QuestProgress {
        quest_id: u32,
        objectives_progress: Vec<u32>,
        started_at: u64,
    }
    
    /// Events emitted by the game
    #[event]
    struct CharacterCreated {
        #[index]
        player: Address,
        name: String,
        class: u8, // 0=Warrior, 1=Rogue, 2=Mage, 3=Ranger, 4=Cleric
    }
    
    #[event]
    struct LevelUp {
        #[index]
        player: Address,
        new_level: u16,
    }
    
    #[event]
    struct ItemAcquired {
        #[index]
        player: Address,
        item_id: u32,
        quantity: u32,
    }
    
    #[event]
    struct QuestStarted {
        #[index]
        player: Address,
        quest_id: u32,
    }
    
    #[event]
    struct QuestCompleted {
        #[index]
        player: Address,
        quest_id: u32,
        reward_exp: u64,
        reward_gold: u64,
    }
    
    #[event]
    struct CombatResult {
        #[index]
        player: Address,
        monster_id: u32,
        victory: bool,
        reward_exp: u64,
        reward_gold: u64,
    }
    
    /// Game storage
    #[storage]
    struct NeoRpg {
        /// Game admin
        admin: Item<Address>,
        
        /// Maps player address to their character
        characters: Map<Address, Character>,
        
        /// Maps item ID to item definition
        items: Map<u32, Item>,
        
        /// Next available item ID
        next_item_id: Item<u32>,
        
        /// Maps quest ID to quest definition
        quests: Map<u32, Quest>,
        
        /// Next available quest ID
        next_quest_id: Item<u32>,
        
        /// Maps monster ID to monster definition
        monsters: Map<u32, Monster>,
        
        /// Next available monster ID
        next_monster_id: Item<u32>,
        
        /// Maps player to quest progress
        quest_progress: Map<(Address, u32), QuestProgress>,
        
        /// Maps player to active combat session
        combat_sessions: Map<Address, CombatSession>,
        
        /// Experience required for each level (level -> exp)
        level_exp_requirements: Map<u16, u64>,
        
        /// Game-wide settings
        max_level: Item<u16>,
        max_inventory: Item<u32>,
        base_attack_cooldown: Item<u64>, // seconds
    }
    
    impl NeoRpg {
        /// Initialize the game contract
        #[constructor]
        fn new(admin: Address) -> Self {
            let mut instance = Self {
                admin: Item::new(admin),
                characters: Map::new(),
                items: Map::new(),
                next_item_id: Item::new(1),
                quests: Map::new(),
                next_quest_id: Item::new(1),
                monsters: Map::new(),
                next_monster_id: Item::new(1),
                quest_progress: Map::new(),
                combat_sessions: Map::new(),
                level_exp_requirements: Map::new(),
                max_level: Item::new(50),
                max_inventory: Item::new(100),
                base_attack_cooldown: Item::new(10), // 10 seconds
            };
            
            // Initialize level experience requirements
            let max_level = *instance.max_level.get();
            for level in 1..=max_level {
                // Simple exponential formula: 100 * level^2
                let exp_required = 100 * (level as u64) * (level as u64);
                instance.level_exp_requirements.insert(level, exp_required);
            }
            
            instance
        }
        
        /// Create a new character
        #[method]
        fn create_character(&mut self, name: String, class_id: u8) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player already has a character
            assert!(!self.characters.contains_key(&player), "Player already has a character");
            
            // Validate name length
            assert!(name.len() >= 3 && name.len() <= 20, "Name must be between 3 and 20 characters");
            
            // Convert class_id to Class enum
            let class = match class_id {
                0 => Class::Warrior,
                1 => Class::Rogue,
                2 => Class::Mage,
                3 => Class::Ranger,
                4 => Class::Cleric,
                _ => panic!("Invalid class ID"),
            };
            
            // Set initial stats based on class
            let stats = match class {
                Class::Warrior => Stats {
                    strength: 10,
                    dexterity: 5,
                    intelligence: 3,
                    vitality: 8,
                    luck: 4,
                },
                Class::Rogue => Stats {
                    strength: 5,
                    dexterity: 10,
                    intelligence: 5,
                    vitality: 5,
                    luck: 7,
                },
                Class::Mage => Stats {
                    strength: 3,
                    dexterity: 5,
                    intelligence: 10,
                    vitality: 4,
                    luck: 6,
                },
                Class::Ranger => Stats {
                    strength: 6,
                    dexterity: 9,
                    intelligence: 5,
                    vitality: 6,
                    luck: 5,
                },
                Class::Cleric => Stats {
                    strength: 5,
                    dexterity: 4,
                    intelligence: 8,
                    vitality: 9,
                    luck: 5,
                },
            };
            
            // Calculate health and mana based on stats
            let max_health = 50 + (stats.vitality as u32 * 10);
            let max_mana = 30 + (stats.intelligence as u32 * 10);
            
            // Create character
            let character = Character {
                name,
                class,
                level: 1,
                experience: 0,
                gold: 100, // Starting gold
                stats,
                health: max_health,
                max_health,
                mana: max_mana,
                max_mana,
                inventory: Vec::new(),
                equipped_weapon: None,
                equipped_armor: None,
                equipped_accessory: None,
                completed_quests: Vec::new(),
                active_quests: Vec::new(),
                created_at: runtime::time(),
                last_action: runtime::time(),
            };
            
            // Add starter items based on class
            let starter_weapon_id = match class {
                Class::Warrior => self.get_or_create_starter_item("Rusty Sword", ItemType::Weapon, Class::Warrior),
                Class::Rogue => self.get_or_create_starter_item("Worn Dagger", ItemType::Weapon, Class::Rogue),
                Class::Mage => self.get_or_create_starter_item("Apprentice Wand", ItemType::Weapon, Class::Mage),
                Class::Ranger => self.get_or_create_starter_item("Simple Bow", ItemType::Weapon, Class::Ranger),
                Class::Cleric => self.get_or_create_starter_item("Wooden Staff", ItemType::Weapon, Class::Cleric),
            };
            
            // Create starter armor
            let starter_armor_id = self.get_or_create_starter_item("Tattered Clothes", ItemType::Armor, Class::Warrior);
            
            // Store character
            self.characters.insert(player, character);
            
            // Add starter items to inventory
            self.add_item_to_player(&player, starter_weapon_id, 1);
            self.add_item_to_player(&player, starter_armor_id, 1);
            
            // Equip starter items
            self.equip_item(&player, starter_weapon_id);
            self.equip_item(&player, starter_armor_id);
            
            // Emit event
            self.emit(CharacterCreated {
                player,
                name: self.characters.get(&player).unwrap().name.clone(),
                class: class_id,
            });
            
            true
        }
        
        /// Add a new item to the game (admin only)
        #[method]
        fn add_item(
            &mut self,
            name: String,
            item_type_id: u8,
            rarity_id: u8,
            level_req: u16,
            class_req_id: Option<u8>,
            strength: Option<u16>,
            dexterity: Option<u16>,
            intelligence: Option<u16>,
            vitality: Option<u16>,
            luck: Option<u16>,
            value: u64,
        ) -> u32 {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.admin.get(), "Only admin can add items");
            
            // Convert item type ID to enum
            let item_type = match item_type_id {
                0 => ItemType::Weapon,
                1 => ItemType::Armor,
                2 => ItemType::Accessory,
                3 => ItemType::Consumable,
                4 => ItemType::QuestItem,
                _ => panic!("Invalid item type ID"),
            };
            
            // Convert rarity ID to enum
            let rarity = match rarity_id {
                0 => Rarity::Common,
                1 => Rarity::Uncommon,
                2 => Rarity::Rare,
                3 => Rarity::Epic,
                4 => Rarity::Legendary,
                _ => panic!("Invalid rarity ID"),
            };
            
            // Convert class requirement
            let class_req = if let Some(class_id) = class_req_id {
                Some(match class_id {
                    0 => Class::Warrior,
                    1 => Class::Rogue,
                    2 => Class::Mage,
                    3 => Class::Ranger,
                    4 => Class::Cleric,
                    _ => panic!("Invalid class ID"),
                })
            } else {
                None
            };
            
            // Create stats if provided
            let stats = if strength.is_some() || dexterity.is_some() || intelligence.is_some() || vitality.is_some() || luck.is_some() {
                Some(Stats {
                    strength: strength.unwrap_or(0),
                    dexterity: dexterity.unwrap_or(0),
                    intelligence: intelligence.unwrap_or(0),
                    vitality: vitality.unwrap_or(0),
                    luck: luck.unwrap_or(0),
                })
            } else {
                None
            };
            
            // Get next item ID
            let item_id = *self.next_item_id.get();
            self.next_item_id.set(item_id + 1);
            
            // Create and store item
            let item = Item {
                id: item_id,
                name,
                item_type,
                rarity,
                level_req,
                class_req,
                stats,
                value,
            };
            
            self.items.insert(item_id, item);
            
            item_id
        }
        
        /// Add a new quest to the game (admin only)
        #[method]
        fn add_quest(
            &mut self,
            name: String,
            description: String,
            level_req: u16,
            prerequisite_quests: Vec<u32>,
            objective_types: Vec<u8>,
            objective_targets: Vec<u32>,
            objective_amounts: Vec<u32>,
            reward_experience: u64,
            reward_gold: u64,
            reward_item_ids: Vec<u32>,
            reward_item_quantities: Vec<u32>,
        ) -> u32 {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.admin.get(), "Only admin can add quests");
            
            // Validate inputs
            assert!(
                objective_types.len() == objective_targets.len() && 
                objective_types.len() == objective_amounts.len(),
                "Objective arrays must have same length"
            );
            
            assert!(
                reward_item_ids.len() == reward_item_quantities.len(),
                "Reward item arrays must have same length"
            );
            
            // Create objectives
            let mut objectives = Vec::new();
            for i in 0..objective_types.len() {
                let objective_type = match objective_types[i] {
                    0 => ObjectiveType::KillMonsters,
                    1 => ObjectiveType::CollectItems,
                    2 => ObjectiveType::CompleteQuest,
                    _ => panic!("Invalid objective type"),
                };
                
                objectives.push(QuestObjective {
                    objective_type,
                    target_id: objective_targets[i],
                    amount: objective_amounts[i],
                });
            }
            
            // Create reward items
            let mut reward_items = Vec::new();
            for i in 0..reward_item_ids.len() {
                reward_items.push((reward_item_ids[i], reward_item_quantities[i]));
            }
            
            // Get next quest ID
            let quest_id = *self.next_quest_id.get();
            self.next_quest_id.set(quest_id + 1);
            
            // Create and store quest
            let quest = Quest {
                id: quest_id,
                name,
                description,
                level_req,
                prerequisite_quests,
                objectives,
                reward_experience,
                reward_gold,
                reward_items,
            };
            
            self.quests.insert(quest_id, quest);
            
            quest_id
        }
        
        /// Add a new monster to the game (admin only)
        #[method]
        fn add_monster(
            &mut self,
            name: String,
            level: u16,
            health: u32,
            attack: u16,
            defense: u16,
            experience_reward: u64,
            gold_reward: u64,
            drop_item_ids: Vec<u32>,
            drop_chances: Vec<u8>,
        ) -> u32 {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.admin.get(), "Only admin can add monsters");
            
            // Validate inputs
            assert!(
                drop_item_ids.len() == drop_chances.len(),
                "Drop arrays must have same length"
            );
            
            // Create drop items
            let mut item_drops = Vec::new();
            for i in 0..drop_item_ids.len() {
                item_drops.push((drop_item_ids[i], drop_chances[i]));
            }
            
            // Get next monster ID
            let monster_id = *self.next_monster_id.get();
            self.next_monster_id.set(monster_id + 1);
            
            // Create and store monster
            let monster = Monster {
                id: monster_id,
                name,
                level,
                health,
                attack,
                defense,
                experience_reward,
                gold_reward,
                item_drops,
            };
            
            self.monsters.insert(monster_id, monster);
            
            monster_id
        }
        
        /// Start a quest
        #[method]
        fn start_quest(&mut self, quest_id: u32) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if quest exists
            let quest = self.quests.get(&quest_id).expect("Quest not found");
            
            // Check if player has a character
            let character = self.characters.get(&player).expect("Character not found");
            
            // Check if player meets level requirement
            assert!(character.level >= quest.level_req, "Level requirement not met");
            
            // Check if player has completed prerequisite quests
            for prereq_id in &quest.prerequisite_quests {
                assert!(
                    character.completed_quests.contains(prereq_id),
                    "Prerequisite quest not completed"
                );
            }
            
            // Check if player already has the quest active
            assert!(
                !character.active_quests.contains(&quest_id),
                "Quest already active"
            );
            
            // Check if player already completed the quest
            assert!(
                !character.completed_quests.contains(&quest_id),
                "Quest already completed"
            );
            
            // Initialize quest progress
            let progress = QuestProgress {
                quest_id,
                objectives_progress: vec![0; quest.objectives.len()],
                started_at: runtime::time(),
            };
            
            // Update character active quests
            let mut updated_character = character.clone();
            updated_character.active_quests.push(quest_id);
            self.characters.insert(player, updated_character);
            
            // Store quest progress
            self.quest_progress.insert((player, quest_id), progress);
            
            // Emit event
            self.emit(QuestStarted {
                player,
                quest_id,
            });
            
            true
        }
        
        /// Update quest progress
        #[method]
        fn update_quest_progress(&mut self, quest_id: u32, objective_index: u32, progress: u32) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player has the quest active
            let character = self.characters.get(&player).expect("Character not found");
            assert!(
                character.active_quests.contains(&quest_id),
                "Quest not active"
            );
            
            // Get quest and progress
            let quest = self.quests.get(&quest_id).expect("Quest not found");
            let mut quest_progress = self.quest_progress.get(&(player, quest_id)).expect("Quest progress not found");
            
            // Validate objective index
            assert!(
                objective_index < quest.objectives.len() as u32,
                "Invalid objective index"
            );
            
            // Update progress
            quest_progress.objectives_progress[objective_index as usize] = progress;
            
            // Store updated progress
            self.quest_progress.insert((player, quest_id), quest_progress.clone());
            
            // Check if quest is complete
            let is_complete = self.check_quest_completion(&player, &quest_id);
            
            // If complete, process rewards
            if is_complete {
                self.complete_quest(&player, &quest_id);
            }
            
            true
        }
        
        /// Start combat with a monster
        #[method]
        fn start_combat(&mut self, monster_id: u32) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player has a character
            let character = self.characters.get(&player).expect("Character not found");
            
            // Check if player already has an active combat session
            assert!(
                !self.combat_sessions.contains_key(&player),
                "Player already in combat"
            );
            
            // Check if monster exists
            let monster = self.monsters.get(&monster_id).expect("Monster not found");
            
            // Create combat session
            let combat_session = CombatSession {
                character_id: player,
                monster_id,
                monster_current_health: monster.health,
                combat_log: Vec::new(),
                started_at: runtime::time(),
                status: CombatStatus::InProgress,
            };
            
            // Store combat session
            self.combat_sessions.insert(player, combat_session);
            
            true
        }
        
        /// Perform attack in combat
        #[method]
        fn attack(&mut self) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player has an active combat session
            let mut combat_session = self.combat_sessions.get(&player).expect("No active combat");
            assert!(
                combat_session.status == CombatStatus::InProgress,
                "Combat already ended"
            );
            
            // Get character and monster
            let mut character = self.characters.get(&player).expect("Character not found");
            let monster = self.monsters.get(&combat_session.monster_id).expect("Monster not found");
            
            // Check cooldown
            let cooldown = *self.base_attack_cooldown.get();
            assert!(
                runtime::time() >= character.last_action + cooldown,
                "Attack on cooldown"
            );
            
            // Calculate character attack
            let attack_value = self.calculate_attack_damage(&character, &monster);
            let critical = self.roll_critical(&character);
            
            if critical {
                let attack_value = attack_value * 2;
            }
            
            // Apply damage to monster
            if attack_value >= combat_session.monster_current_health {
                // Monster defeated
                combat_session.monster_current_health = 0;
                combat_session.status = CombatStatus::Victory;
            } else {
                combat_session.monster_current_health -= attack_value;
            }
            
            // Log attack
            combat_session.combat_log.push(CombatLogEntry {
                attacker: character.name.clone(),
                defender: monster.name.clone(),
                damage: attack_value,
                critical,
                timestamp: runtime::time(),
            });
            
            // If monster still alive, it counter-attacks
            if combat_session.status == CombatStatus::InProgress {
                // Calculate monster attack
                let monster_attack = self.calculate_monster_damage(&character, &monster);
                
                // Apply damage to character
                if monster_attack >= character.health {
                    // Character defeated
                    character.health = 0;
                    combat_session.status = CombatStatus::Defeat;
                } else {
                    character.health -= monster_attack;
                }
                
                // Log monster attack
                combat_session.combat_log.push(CombatLogEntry {
                    attacker: monster.name.clone(),
                    defender: character.name.clone(),
                    damage: monster_attack,
                    critical: false,
                    timestamp: runtime::time(),
                });
            }
            
            // Update last action timestamp
            character.last_action = runtime::time();
            
            // If combat ended, process results
            if combat_session.status != CombatStatus::InProgress {
                self.process_combat_result(&player, &combat_session);
            }
            
            // Store updated character and combat session
            self.characters.insert(player, character);
            self.combat_sessions.insert(player, combat_session);
            
            true
        }
        
        /// Equip an item
        #[method]
        fn equip_item(&mut self, item_id: u32) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player has a character
            let mut character = self.characters.get(&player).expect("Character not found");
            
            // Check if player has the item
            let has_item = character.inventory.iter().any(|inv_item| inv_item.item_id == item_id);
            assert!(has_item, "Item not in inventory");
            
            // Get item definition
            let item = self.items.get(&item_id).expect("Item not found");
            
            // Check level requirement
            assert!(character.level >= item.level_req, "Level requirement not met");
            
            // Check class requirement
            if let Some(class_req) = &item.class_req {
                assert!(character.class == *class_req, "Class requirement not met");
            }
            
            // Handle equipping based on item type
            match item.item_type {
                ItemType::Weapon => {
                    // Unequip current weapon if any
                    if let Some(current_weapon) = character.equipped_weapon {
                        for inv_item in &mut character.inventory {
                            if inv_item.item_id == current_weapon {
                                inv_item.equipped = false;
                                break;
                            }
                        }
                    }
                    
                    // Equip new weapon
                    character.equipped_weapon = Some(item_id);
                }
                ItemType::Armor => {
                    // Unequip current armor if any
                    if let Some(current_armor) = character.equipped_armor {
                        for inv_item in &mut character.inventory {
                            if inv_item.item_id == current_armor {
                                inv_item.equipped = false;
                                break;
                            }
                        }
                    }
                    
                    // Equip new armor
                    character.equipped_armor = Some(item_id);
                }
                ItemType::Accessory => {
                    // Unequip current accessory if any
                    if let Some(current_accessory) = character.equipped_accessory {
                        for inv_item in &mut character.inventory {
                            if inv_item.item_id == current_accessory {
                                inv_item.equipped = false;
                                break;
                            }
                        }
                    }
                    
                    // Equip new accessory
                    character.equipped_accessory = Some(item_id);
                }
                _ => panic!("Cannot equip this item type"),
            }
            
            // Mark item as equipped in inventory
            for inv_item in &mut character.inventory {
                if inv_item.item_id == item_id {
                    inv_item.equipped = true;
                    break;
                }
            }
            
            // Recalculate character stats
            self.update_character_stats(&mut character);
            
            // Store updated character
            self.characters.insert(player, character);
            
            true
        }
        
        /// Use a consumable item
        #[method]
        fn use_item(&mut self, item_id: u32) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player has a character
            let mut character = self.characters.get(&player).expect("Character not found");
            
            // Find item in inventory
            let mut item_index = None;
            for (i, inv_item) in character.inventory.iter().enumerate() {
                if inv_item.item_id == item_id {
                    item_index = Some(i);
                    break;
                }
            }
            
            let inv_index = item_index.expect("Item not in inventory");
            
            // Get item definition
            let item = self.items.get(&item_id).expect("Item not found");
            
            // Check if item is consumable
            assert!(item.item_type == ItemType::Consumable, "Item is not consumable");
            
            // Apply item effects (simplified for example)
            if let Some(stats) = &item.stats {
                // For this example, consumables restore health and mana based on vitality/intelligence values
                if stats.vitality > 0 {
                    let health_restore = stats.vitality as u32 * 10;
                    character.health = u32::min(character.health + health_restore, character.max_health);
                }
                
                if stats.intelligence > 0 {
                    let mana_restore = stats.intelligence as u32 * 10;
                    character.mana = u32::min(character.mana + mana_restore, character.max_mana);
                }
            }
            
            // Consume the item (reduce quantity or remove if last one)
            let inv_item = &mut character.inventory[inv_index];
            inv_item.quantity -= 1;
            
            if inv_item.quantity == 0 {
                character.inventory.remove(inv_index);
            }
            
            // Store updated character
            self.characters.insert(player, character);
            
            true
        }
        
        /// Rest to restore health and mana
        #[method]
        fn rest(&mut self) -> bool {
            let player = runtime::calling_script_hash();
            
            // Verify player signature
            assert!(runtime::check_witness(&player), "Invalid signature");
            
            // Check if player has a character
            let mut character = self.characters.get(&player).expect("Character not found");
            
            // Check if player is not in combat
            assert!(
                !self.combat_sessions.contains_key(&player) || 
                self.combat_sessions.get(&player).unwrap().status != CombatStatus::InProgress,
                "Cannot rest while in combat"
            );
            
            // Restore 25% of health and mana
            let health_restore = character.max_health / 4;
            let mana_restore = character.max_mana / 4;
            
            character.health = u32::min(character.health + health_restore, character.max_health);
            character.mana = u32::min(character.mana + mana_restore, character.max_mana);
            
            // Update last action timestamp
            character.last_action = runtime::time();
            
            // Store updated character
            self.characters.insert(player, character);
            
            true
        }
        
        /// Get character info
        #[safe]
        fn get_character(&self, player: Address) -> Option<(
            String, u8, u16, u64, u64, u32, u32, u32, u32, u32, u32, u32
        )> {
            let character = self.characters.get(&player)?;
            
            // Return simplified character info
            Some((
                character.name.clone(),
                self.class_to_id(&character.class),
                character.level,
                character.experience,
                character.gold,
                character.stats.strength as u32,
                character.stats.dexterity as u32,
                character.stats.intelligence as u32,
                character.stats.vitality as u32,
                character.stats.luck as u32,
                character.health,
                character.mana,
            ))
        }
        
        /// Get character inventory
        #[safe]
        fn get_inventory(&self, player: Address) -> Vec<(u32, u32, bool)> {
            let character = match self.characters.get(&player) {
                Some(c) => c,
                None => return Vec::new(),
            };
            
            // Return list of (item_id, quantity, equipped)
            character.inventory.iter()
                .map(|item| (item.item_id, item.quantity, item.equipped))
                .collect()
        }
        
        /// Get item details
        #[safe]
        fn get_item(&self, item_id: u32) -> Option<(
            String, u8, u8, u16, Option<u8>, Option<u16>, Option<u16>, Option<u16>, Option<u16>, Option<u16>, u64
        )> {
            let item = self.items.get(&item_id)?;
            
            // Return item details
            Some((
                item.name.clone(),
                self.item_type_to_id(&item.item_type),
                self.rarity_to_id(&item.rarity),
                item.level_req,
                item.class_req.as_ref().map(|c| self.class_to_id(c)),
                item.stats.as_ref().map(|s| s.strength),
                item.stats.as_ref().map(|s| s.dexterity),
                item.stats.as_ref().map(|s| s.intelligence),
                item.stats.as_ref().map(|s| s.vitality),
                item.stats.as_ref().map(|s| s.luck),
                item.value,
            ))
        }
        
        /// Get quest details
        #[safe]
        fn get_quest(&self, quest_id: u32) -> Option<(
            String, String, u16, u64, u64
        )> {
            let quest = self.quests.get(&quest_id)?;
            
            // Return simplified quest info
            Some((
                quest.name.clone(),
                quest.description.clone(),
                quest.level_req,
                quest.reward_experience,
                quest.reward_gold,
            ))
        }
        
        /// Get quest objectives
        #[safe]
        fn get_quest_objectives(&self, quest_id: u32) -> Vec<(u8, u32, u32)> {
            let quest = match self.quests.get(&quest_id) {
                Some(q) => q,
                None => return Vec::new(),
            };
            
            // Return list of (objective_type, target_id, amount)
            quest.objectives.iter()
                .map(|obj| (
                    self.objective_type_to_id(&obj.objective_type),
                    obj.target_id,
                    obj.amount
                ))
                .collect()
        }
        
        /// Get quest progress for a player
        #[safe]
        fn get_quest_progress(&self, player: Address, quest_id: u32) -> Option<Vec<u32>> {
            let progress = self.quest_progress.get(&(player, quest_id))?;
            
            Some(progress.objectives_progress.clone())
        }
        
        /// Get active combat session
        #[safe]
        fn get_combat_session(&self, player: Address) -> Option<(
            u32, u32, u8, Vec<(String, String, u32, bool)>
        )> {
            let session = self.combat_sessions.get(&player)?;
            
            // Convert combat log to simplified format
            let log: Vec<(String, String, u32, bool)> = session.combat_log.iter()
                .map(|entry| (
                    entry.attacker.clone(),
                    entry.defender.clone(),
                    entry.damage,
                    entry.critical
                ))
                .collect();
            
            Some((
                session.monster_id,
                session.monster_current_health,
                self.combat_status_to_id(&session.status),
                log,
            ))
        }
        
        // === Helper methods ===
        
        /// Get or create a starter item
        fn get_or_create_starter_item(&mut self, name: &str, item_type: ItemType, class: Class) -> u32 {
            // Check if this starter item already exists
            for (id, item) in self.items.iter() {
                if item.name == name && item.item_type == item_type {
                    return *id;
                }
            }
            
            // Create new starter item
            let item_id = *self.next_item_id.get();
            self.next_item_id.set(item_id + 1);
            
            // Basic stats for starter items
            let stats = match item_type {
                ItemType::Weapon => {
                    Some(Stats {
                        strength: if class == Class::Warrior { 3 } else { 1 },
                        dexterity: if class == Class::Rogue || class == Class::Ranger { 3 } else { 1 },
                        intelligence: if class == Class::Mage || class == Class::Cleric { 3 } else { 1 },
                        vitality: 0,
                        luck: 0,
                    })
                },
                ItemType::Armor => {
                    Some(Stats {
                        strength: 0,
                        dexterity: 0,
                        intelligence: 0,
                        vitality: 2,
                        luck: 0,
                    })
                },
                _ => None,
            };
            
            // Create item
            let item = Item {
                id: item_id,
                name: name.to_string(),
                item_type,
                rarity: Rarity::Common,
                level_req: 1,
                class_req: Some(class.clone()),
                stats,
                value: 10,
            };
            
            self.items.insert(item_id, item);
            
            item_id
        }
        
        /// Add item to player inventory
        fn add_item_to_player(&mut self, player: &Address, item_id: u32, quantity: u32) {
            // Check if player has a character
            let mut character = self.characters.get(player).expect("Character not found");
            
            // Check if player already has this item
            let mut found = false;
            for inv_item in &mut character.inventory {
                if inv_item.item_id == item_id {
                    inv_item.quantity += quantity;
                    found = true;
                    break;
                }
            }
            
            // If not found, add new inventory item
            if !found {
                character.inventory.push(InventoryItem {
                    item_id,
                    quantity,
                    equipped: false,
                });
            }
            
            // Store updated character
            self.characters.insert(*player, character);
            
            // Emit event
            self.emit(ItemAcquired {
                player: *player,
                item_id,
                quantity,
            });
        }
        
        /// Check if a quest is complete
        fn check_quest_completion(&self, player: &Address, quest_id: &u32) -> bool {
            // Get quest and progress
            let quest = self.quests.get(quest_id).expect("Quest not found");
            let progress = self.quest_progress.get(&(*player, *quest_id)).expect("Quest progress not found");
            
            // Check each objective
            for (i, objective) in quest.objectives.iter().enumerate() {
                if progress.objectives_progress[i] < objective.amount {
                    return false;
                }
            }
            
            true
        }
        
        /// Complete a quest and grant rewards
        fn complete_quest(&mut self, player: &Address, quest_id: &u32) {
            // Get character and quest
            let mut character = self.characters.get(player).expect("Character not found");
            let quest = self.quests.get(quest_id).expect("Quest not found");
            
            // Remove from active quests
            if let Some(index) = character.active_quests.iter().position(|id| id == quest_id) {
                character.active_quests.remove(index);
            }
            
            // Add to completed quests
            character.completed_quests.push(*quest_id);
            
            // Grant experience reward
            self.grant_experience(player, &mut character, quest.reward_experience);
            
            // Grant gold reward
            character.gold += quest.reward_gold;
            
            // Grant item rewards
            for (item_id, quantity) in &quest.reward_items {
                self.add_item_to_player(player, *item_id, *quantity);
            }
            
            // Store updated character
            self.characters.insert(*player, character);
            
            // Emit event
            self.emit(QuestCompleted {
                player: *player,
                quest_id: *quest_id,
                reward_exp: quest.reward_experience,
                reward_gold: quest.reward_gold,
            });
        }
        
        /// Update character stats based on equipped items
        fn update_character_stats(&mut self, character: &mut Character) {
            // Reset character to base stats for their level
            let base_stats = self.calculate_base_stats(character.class, character.level);
            character.stats = base_stats.clone();
            
            // Calculate max health and mana from base stats
            character.max_health = 50 + (character.level as u32 * 10) + (character.stats.vitality as u32 * 10);
            character.max_mana = 30 + (character.level as u32 * 5) + (character.stats.intelligence as u32 * 10);
            
            // Add stats from equipped items
            let equipped_items = [character.equipped_weapon, character.equipped_armor, character.equipped_accessory];
            
            for item_id_opt in equipped_items.iter().filter_map(|id| *id) {
                if let Some(item) = self.items.get(&item_id_opt) {
                    if let Some(item_stats) = &item.stats {
                        character.stats.strength += item_stats.strength;
                        character.stats.dexterity += item_stats.dexterity;
                        character.stats.intelligence += item_stats.intelligence;
                        character.stats.vitality += item_stats.vitality;
                        character.stats.luck += item_stats.luck;
                    }
                }
            }
            
            // Recalculate max health and mana with equipped items
            character.max_health = 50 + (character.level as u32 * 10) + (character.stats.vitality as u32 * 10);
            character.max_mana = 30 + (character.level as u32 * 5) + (character.stats.intelligence as u32 * 10);
            
            // Ensure current health/mana don't exceed max
            character.health = u32::min(character.health, character.max_health);
            character.mana = u32::min(character.mana, character.max_mana);
        }
        
        /// Calculate base stats for a character class and level
        fn calculate_base_stats(&self, class: Class, level: u16) -> Stats {
            // Set base stats based on class
            let mut stats = match class {
                Class::Warrior => Stats {
                    strength: 10,
                    dexterity: 5,
                    intelligence: 3,
                    vitality: 8,
                    luck: 4,
                },
                Class::Rogue => Stats {
                    strength: 5,
                    dexterity: 10,
                    intelligence: 5,
                    vitality: 5,
                    luck: 7,
                },
                Class::Mage => Stats {
                    strength: 3,
                    dexterity: 5,
                    intelligence: 10,
                    vitality: 4,
                    luck: 6,
                },
                Class::Ranger => Stats {
                    strength: 6,
                    dexterity: 9,
                    intelligence: 5,
                    vitality: 6,
                    luck: 5,
                },
                Class::Cleric => Stats {
                    strength: 5,
                    dexterity: 4,
                    intelligence: 8,
                    vitality: 9,
                    luck: 5,
                },
            };
            
            // Add stat points for level (simplified: 2 points per level after 1)
            let level_points = (level - 1) as u16 * 2;
            
            // Distribute based on class focus
            match class {
                Class::Warrior => {
                    stats.strength += level_points / 2;
                    stats.vitality += level_points / 4;
                    stats.dexterity += level_points / 4;
                },
                Class::Rogue => {
                    stats.dexterity += level_points / 2;
                    stats.luck += level_points / 4;
                    stats.strength += level_points / 4;
                },
                Class::Mage => {
                    stats.intelligence += level_points / 2;
                    stats.luck += level_points / 4;
                    stats.vitality += level_points / 4;
                },
                Class::Ranger => {
                    stats.dexterity += level_points / 2;
                    stats.strength += level_points / 4;
                    stats.intelligence += level_points / 4;
                },
                Class::Cleric => {
                    stats.intelligence += level_points / 2;
                    stats.vitality += level_points / 4;
                    stats.wisdom += level_points / 4;
                },
            }
            
            stats
        }
        
        /// Calculate attack damage by character against monster
        fn calculate_attack_damage(&self, character: &Character, monster: &Monster) -> u32 {
            // Get weapon damage if equipped
            let weapon_damage = if let Some(weapon_id) = character.equipped_weapon {
                if let Some(weapon) = self.items.get(&weapon_id) {
                    if let Some(stats) = &weapon.stats {
                        // For simplicity, weapon damage is based on primary stat
                        stats.strength as u32 * 5
                    } else {
                        5 // Default damage
                    }
                } else {
                    5 // Default damage
                }
            } else {
                5 // Unarmed damage
            };
            
            // Calculate base damage based on class
            let base_damage = match character.class {
                Class::Warrior => character.stats.strength as u32 * 2 + character.level as u32 * 3,
                Class::Rogue => character.stats.dexterity as u32 * 2 + character.level as u32 * 2,
                Class::Mage => character.stats.intelligence as u32 * 2 + character.level as u32 * 2,
                Class::Ranger => character.stats.dexterity as u32 * 2 + character.level as u32 * 2,
                Class::Cleric => character.stats.intelligence as u32 + character.stats.strength as u32 + character.level as u32 * 2,
            };
            
            // Calculate total damage
            let total_damage = base_damage + weapon_damage;
            
            // Apply defense reduction
            let defense_reduction = u32::min(total_damage, monster.defense as u32);
            let final_damage = total_damage - defense_reduction;
            
            final_damage
        }
        
        /// Calculate monster damage against character
        fn calculate_monster_damage(&self, character: &Character, monster: &Monster) -> u32 {
            // Base monster damage
            let base_damage = monster.attack as u32 * (1 + monster.level as u32 / 5);
            
            // Get armor defense if equipped
            let armor_defense = if let Some(armor_id) = character.equipped_armor {
                if let Some(armor) = self.items.get(&armor_id) {
                    if let Some(stats) = &armor.stats {
                        // For simplicity, armor defense is based on vitality
                        stats.vitality as u32 * 3
                    } else {
                        0 // No defense
                    }
                } else {
                    0 // No defense
                }
            } else {
                0 // No armor
            };
            
            // Calculate character defense based on stats
            let character_defense = character.stats.vitality as u32 + armor_defense;
            
            // Apply defense reduction
            let defense_reduction = u32::min(base_damage, character_defense);
            let final_damage = base_damage - defense_reduction;
            
            final_damage
        }
        
        /// Roll for critical hit based on luck
        fn roll_critical(&self, character: &Character) -> bool {
            // Get pseudo-random value based on block hash + timestamp
            let block = runtime::get_block();
            let timestamp = runtime::time();
            
            // Create "randomness" by hashing block hash with timestamp
            let mut seed_bytes = Vec::new();
            seed_bytes.extend_from_slice(&block.hash);
            seed_bytes.extend_from_slice(&timestamp.to_ne_bytes());
            
            // Get last byte as random value 0-255
            let random_value = seed_bytes[seed_bytes.len() - 1] as u16;
            
            // Critical chance based on luck (1% per point)
            let crit_chance = character.stats.luck;
            
            // If random value is less than crit chance, it's a critical hit
            random_value < crit_chance
        }
        
        /// Process combat results
        fn process_combat_result(&mut self, player: &Address, combat_session: &CombatSession) {
            // Get character
            let mut character = self.characters.get(player).expect("Character not found");
            
            if combat_session.status == CombatStatus::Victory {
                // Get monster
                let monster = self.monsters.get(&combat_session.monster_id).expect("Monster not found");
                
                // Grant experience
                self.grant_experience(player, &mut character, monster.experience_reward);
                
                // Grant gold
                character.gold += monster.gold_reward;
                
                // Check for item drops
                for (item_id, drop_chance) in &monster.item_drops {
                    // Simple "random" drop mechanism
                    let block = runtime::get_block();
                    let timestamp = runtime::time();
                    
                    // Create "randomness" by hashing block hash with timestamp and item
                    let mut seed_bytes = Vec::new();
                    seed_bytes.extend_from_slice(&block.hash);
                    seed_bytes.extend_from_slice(&timestamp.to_ne_bytes());
                    seed_bytes.extend_from_slice(&item_id.to_ne_bytes());
                    
                    // Get last byte as random value 0-255
                    let random_value = seed_bytes[seed_bytes.len() - 1] as u8;
                    
                    // If random value is less than drop chance, the item drops
                    if random_value < *drop_chance {
                        self.add_item_to_player(player, *item_id, 1);
                    }
                }
                
                // Update quest progress for monster kills if needed
                for quest_id in &character.active_quests {
                    if let Some(quest) = self.quests.get(quest_id) {
                        if let Some(progress) = self.quest_progress.get(&(*player, *quest_id)) {
                            let mut updated_progress = progress.clone();
                            let mut updated = false;
                            
                            // Check each objective
                            for (i, objective) in quest.objectives.iter().enumerate() {
                                if objective.objective_type == ObjectiveType::KillMonsters 
                                    && objective.target_id == combat_session.monster_id {
                                    
                                    updated_progress.objectives_progress[i] += 1;
                                    updated = true;
                                }
                            }
                            
                            if updated {
                                // Store updated progress
                                self.quest_progress.insert((*player, *quest_id), updated_progress.clone());
                                
                                // Check if quest is complete
                                let is_complete = self.check_quest_completion(player, quest_id);
                                
                                // If complete, process rewards
                                if is_complete {
                                    self.complete_quest(player, quest_id);
                                }
                            }
                        }
                    }
                }
                
                // Emit combat result event
                self.emit(CombatResult {
                    player: *player,
                    monster_id: combat_session.monster_id,
                    victory: true,
                    reward_exp: monster.experience_reward,
                    reward_gold: monster.gold_reward,
                });
            } else {
                // Character was defeated
                // Set health to 1 (merciful game mechanic)
                character.health = 1;
                
                // Apply defeat penalty - lose 10% of gold
                let gold_loss = character.gold / 10;
                character.gold -= gold_loss;
                
                // Emit combat result event
                self.emit(CombatResult {
                    player: *player,
                    monster_id: combat_session.monster_id,
                    victory: false,
                    reward_exp: 0,
                    reward_gold: 0,
                });
            }
            
            // Remove combat session
            self.combat_sessions.remove(player);
            
            // Store updated character
            self.characters.insert(*player, character);
        }
        
        /// Grant experience to a character and handle level ups
        fn grant_experience(&mut self, player: &Address, character: &mut Character, amount: u64) {
            // Add experience
            character.experience += amount;
            
            // Check for level up
            let max_level = *self.max_level.get();
            
            while character.level < max_level {
                // Get experience required for next level
                let next_level = character.level + 1;
                let exp_required = self.level_exp_requirements.get(&next_level).unwrap_or_default();
                
                // If enough exp, level up
                if character.experience >= exp_required {
                    character.level = next_level;
                    
                    // Update stats for new level
                    self.update_character_stats(character);
                    
                    // Fully restore health and mana on level up
                    character.health = character.max_health;
                    character.mana = character.max_mana;
                    
                    // Emit level up event
                    self.emit(LevelUp {
                        player: *player,
                        new_level: character.level,
                    });
                } else {
                    // Not enough exp for next level
                    break;
                }
            }
        }
        
        // Utility methods to convert enums to IDs for external interfaces
        
        fn class_to_id(&self, class: &Class) -> u8 {
            match class {
                Class::Warrior => 0,
                Class::Rogue => 1,
                Class::Mage => 2,
                Class::Ranger => 3,
                Class::Cleric => 4,
            }
        }
        
        fn item_type_to_id(&self, item_type: &ItemType) -> u8 {
            match item_type {
                ItemType::Weapon => 0,
                ItemType::Armor => 1,
                ItemType::Accessory => 2,
                ItemType::Consumable => 3,
                ItemType::QuestItem => 4,
            }
        }
        
        fn rarity_to_id(&self, rarity: &Rarity) -> u8 {
            match rarity {
                Rarity::Common => 0,
                Rarity::Uncommon => 1,
                Rarity::Rare => 2,
                Rarity::Epic => 3,
                Rarity::Legendary => 4,
            }
        }
        
        fn objective_type_to_id(&self, objective_type: &ObjectiveType) -> u8 {
            match objective_type {
                ObjectiveType::KillMonsters => 0,
                ObjectiveType::CollectItems => 1,
                ObjectiveType::CompleteQuest => 2,
            }
        }
        
        fn combat_status_to_id(&self, status: &CombatStatus) -> u8 {
            match status {
                CombatStatus::InProgress => 0,
                CombatStatus::Victory => 1,
                CombatStatus::Defeat => 2,
            }
        }
    }
}