use crate::state::{read_config, store_proposal, Proposal};
use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response, Uint128};
use governance_types::errors::ContractError;

pub fn execute_vote(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // TODO implement voting, and save state

    Ok(Response::new()
        .add_attribute("action", "execute vote")
        .add_attribute("voter", info.sender.as_str()))
}

pub fn execute_propose(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
) -> Result<Response<Empty>, ContractError> {
    let cfg = read_config(deps.storage)?;

    let prop = Proposal {
        title,
        proposer: info.sender.clone(),
        min_votes: cfg.required_votes,
    };

    store_proposal(deps.storage, &prop)?;
    Ok(Response::new()
        .add_attribute("title", prop.title)
        .add_attribute("proposer", prop.proposer)
        .add_attribute("min_votes", prop.min_votes))
}
