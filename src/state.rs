use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

pub const STATE: Item<State> = Item::new("state");
