use crate::execute::{execute_proposal, execute_vote, execute_vote_status};
use crate::queries::{query_config, query_propose};
use crate::state::{store_config, store_proposal, Config, Proposal};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint128,
};
use governance_types::errors::ContractError;
use governance_types::types::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, ProposalResponse, ProposeMsg, QueryMsg, Status, Vote,
};

// Method is executed when a new contract instance is created. You can treat it as a constructor.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: _info.sender.clone(),
        required_votes: Uint128::from(10u128),
    };
    store_config(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "instatiate")
        .add_attribute("owner", _info.sender)
        .add_attribute("required_votes", config.required_votes))
}

// Methods which are executed when someone send call which changes blockchain state.
// It can be compared to Solidity NON view methods.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    match msg {
        // TODO add required method types and handlers for each.
        ExecuteMsg::Vote { vote, weight } => execute_vote(deps, env, info, vote, weight),
        ExecuteMsg::Propose(ProposeMsg { title, wh }) => {
            execute_proposal(deps, env, info, title, wh)
        }
        ExecuteMsg::CloseVote {} => execute_vote_status(deps, env, info),
    }
}

// Methods which are executed when someone send a query (gas free call).
// It can be compared to Solidity view methods.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        // TODO implement missing even handlers
        QueryMsg::Config {} => Ok(to_binary(&query_config(deps)?)?),
        QueryMsg::GetVoter { .. } => Ok(to_binary(&{})?),
        QueryMsg::GetStatus { .. } => Ok(to_binary(&{})?),
        QueryMsg::GetPropose {} => Ok(to_binary(&query_propose(deps)?)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use governance_types::types::ConfigResponse;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&coins(1000, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!("creator", value.owner);
    }

    #[test]
    fn proper_create_proposal() {
        let mut deps = mock_dependencies(&coins(1000, "token"));

        let info = mock_info("creator", &coins(1000, "propT"));
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let msg = ExecuteMsg::Propose(ProposeMsg {
            title: "New proposal".to_string(),
            wh: true,
        });

        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPropose {}).unwrap();
        let value: ProposalResponse = from_binary(&res).unwrap();
        assert_eq!("New proposal", value.title);
        assert_eq!("creator", value.proposer);
        assert_eq!(Uint128::from(10u128), value.min_votes);
    }

    #[test]
    fn execute_vote() {
        let mut deps = mock_dependencies(&coins(1000, "token"));

        let info = mock_info("creator", &coins(1000, "propT"));
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let msg = ExecuteMsg::Propose(ProposeMsg {
            title: "Proposal for getting a votes".to_string(),
            wh: true,
        });
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        let msg = ExecuteMsg::Vote {
            vote: Vote::Yes,
            weight: Uint128::from(120u128),
        };
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPropose {}).unwrap();
        let value: ProposalResponse = from_binary(&res).unwrap();
        assert_eq!(Uint128::from(10u128), value.min_votes);
        assert_eq!(Uint128::from(120u128), value.total_votes);
    }

    #[test]
    fn closing_proposal() {
        let mut deps = mock_dependencies(&coins(1000, "token"));

        let info = mock_info("creator", &coins(1000, "propT"));
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let msg = ExecuteMsg::Propose(ProposeMsg {
            title: "Proposal with OPEN status".to_string(),
            wh: true,
        });
        //Execute Vote with less than 10 value
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        let msg = ExecuteMsg::Vote {
            vote: Vote::Yes,
            weight: Uint128::from(10u128),
        };
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        //Closing proposal
        let msg = ExecuteMsg::Vote {
            vote: Vote::Yes,
            weight: Uint128::from(9u128),
        };
        let info = mock_info("creator", &coins(1000, "propT"));
        let msg = ExecuteMsg::CloseVote {};
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        //get proposal
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPropose {}).unwrap();
        let value: ProposalResponse = from_binary(&res).unwrap();
        assert!(matches!(value.status, Status::Rejected));
    }

    #[test]
    fn not_allow_user_to_vote_more_than_once() {
        let mut deps = mock_dependencies(&coins(1000, "token"));

        let info = mock_info("creator", &coins(1000, "propT"));
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let msg = ExecuteMsg::Propose(ProposeMsg {
            title: "Block user before voting more than one time".to_string(),
            wh: true,
        });
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        //First voting
        let msg = ExecuteMsg::Vote {
            vote: Vote::Yes,
            weight: Uint128::from(120u128),
        };
        let info = mock_info("voter_address", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        //Second voting
        let msg = ExecuteMsg::Vote {
            vote: Vote::No,
            weight: Uint128::from(20u128),
        };
        let info = mock_info("voter_address", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }

    #[test]
    fn allow_to_vote_only_whitelisted_address() {
        let mut deps = mock_dependencies(&coins(1000, "token"));

        let info = mock_info("creator", &coins(1000, "propT"));
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        //WH = false not allow to vote
        let msg = ExecuteMsg::Propose(ProposeMsg {
            title: "Block user before voting more than one time".to_string(),
            wh: false,
        });
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        let msg = ExecuteMsg::Vote {
            vote: Vote::Yes,
            weight: Uint128::from(120u128),
        };
        let info = mock_info("notWH", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());

        //WH = true not allow to vote
        let msg = ExecuteMsg::Propose(ProposeMsg {
            title: "Block user before voting more than one time".to_string(),
            wh: true,
        });
        let info = mock_info("creator", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        let msg = ExecuteMsg::Vote {
            vote: Vote::Yes,
            weight: Uint128::from(120u128),
        };
        let info = mock_info("WH", &coins(1000, "propT"));
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());
    }
}
