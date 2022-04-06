use cosmwasm_std::Addr;
use cosmwasm_std::{StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// cs-storage-plus docs: https://crates.io/crates/cw-storage-plus

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    //TODO contract admin, voting settings
    pub owner: Addr,
    pub required_votes: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Proposal {
    pub proposer: Addr,
    pub title: String,
    pub min_votes: Uint128,
}

const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config");
const PROPOSALS: Item<Proposal> = Item::new("proposals");

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

pub fn store_proposal(storage: &mut dyn Storage, prop: &Proposal) -> StdResult<()> {
    PROPOSALS.save(storage, &prop)
}

pub fn read_proposal(storage: &dyn Storage) -> StdResult<Proposal> {
    PROPOSALS.load(storage)
}
