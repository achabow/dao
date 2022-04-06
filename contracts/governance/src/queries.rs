use crate::state::{read_config, read_proposal};
use cosmwasm_std::Deps;
use governance_types::types::ConfigResponse;
use governance_types::{errors::ContractError, types::ProposalResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn query_config(deps: Deps) -> Result<ConfigResponse, ContractError> {
    let config = read_config(deps.storage)?;
    let owner = String::from(config.owner);

    let resp = ConfigResponse { owner };

    Ok(resp)
}

pub fn query_propose(deps: Deps) -> Result<ProposalResponse, ContractError> {
    let config = read_config(deps.storage)?;

    let prop = read_proposal(deps.storage)?;
    let title = prop.title;
    let proposer = String::from(prop.proposer);
    let min_votes = config.required_votes;
    let total_votes = prop.votes.total();
    let status = prop.status;

    let resp = ProposalResponse {
        title,
        min_votes,
        proposer,
        total_votes,
        status,
    };

    Ok(resp)
}
