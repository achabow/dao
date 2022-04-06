use crate::state::{read_config, read_proposal, store_proposal, Proposal, Votes};
use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response, Uint128};
use governance_types::errors::ContractError;
use governance_types::types::Vote;

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vote: Vote,
    weight: Uint128,
) -> Result<Response, ContractError> {
    // TODO implement voting, and save state
    let mut prop = read_proposal(deps.storage)?;
    prop.votes.add_vote(vote, weight);

    store_proposal(deps.storage, &prop)?;
    Ok(Response::new()
        .add_attribute("action", "execute vote")
        .add_attribute("voter", info.sender.as_str())
        .add_attribute("votes", prop.votes.total()))
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
        votes: Votes {
            yes: Uint128::zero(),
            no: Uint128::zero(),
            abstain: Uint128::zero(),
        },
    };

    store_proposal(deps.storage, &prop)?;
    Ok(Response::new()
        .add_attribute("title", prop.title)
        .add_attribute("proposer", prop.proposer)
        .add_attribute("min_votes", prop.min_votes))
}
