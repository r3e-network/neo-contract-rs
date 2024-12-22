// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(dead_code)]

use crate::types::*;

pub trait Nep24 {
    fn royalty_info(token_id: ByteString, royalty_token: Int256, sale_price: Int256) -> Map<ByteString, Any>;
}

pub trait Nep26 {
    fn on_nep11_payment(from: H160, amount: Int256, token_id: ByteString);

    // fn on_nep17_payment_with_data(from: H160, amount: Int256, data: Any);
}

pub trait Nep27 {
    fn on_nep17_payment(from: H160, amount: Int256);

    // fn on_nep17_payment_with_data(from: H160, amount: Int256, data: Any);
}

pub trait Nep28 {
    fn _deploy(data: Any, is_update: bool);
}

pub trait Nep30 {
    fn verify(args: Array<Any>) -> bool;
}

pub trait OnOracleResponse {
    fn on_oracle_response(
        request_url: ByteString,
        request_user_data: Any,
        response_code: OracleResponseCode,
        response_json: ByteString,
    );
}
