// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{contract::*, types::*};

pub trait TokenState {
    fn name() -> ByteString;

    fn owner() -> H160;
}

pub trait Nep11Token<T: TokenState>: TokenContract {
    fn owner_of(token_id: ByteString) -> H160;

    fn properties(token_id: ByteString) -> Map<ByteString, Any>;

    // fn tokens() -> Iter<T>;
    //
    // fn tokens_of(owner: H160) -> Iter<T>;

    fn mint(token_id: ByteString, token_state: T);

    fn burn(token_id: ByteString);
}

// pub fn update_nep11_balance(owner: H160, token_id: ByteString, increment: Int256) {
//     let balance = get_nep11_balance(owner, token_id);
//     let new_balance = balance + increment;
//     set_nep11_balance(owner, token_id, new_balance);
// }

