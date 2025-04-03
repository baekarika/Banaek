use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use cw2::set_contract_version;

mod msg;
mod state;
mod execute;
mod query;
use msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use state::{CONTRACT_VERSION, CONTRACT_INFO, ContractInfo};
use execute::*;
use query::*;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_VERSION)?;

    CONTRACT_INFO.save(
        deps.storage,
        &ContractInfo {
            version: CONTRACT_VERSION.to_string(),
            creator: msg.admin.clone(),
        },
    )?;

    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddEquipment { id, name, description, usage_threshold } => {
            execute_add_equipment(deps, id, name, description, usage_threshold)
        }
        ExecuteMsg::LogMaintenance { equipment_id, maintenance_type, notes } => {
            execute_log_maintenance(deps, env, equipment_id, maintenance_type, notes)
        }
        ExecuteMsg::UpdateThreshold { equipment_id, new_threshold } => {
            execute_update_threshold(deps, equipment_id, new_threshold)
        }
        ExecuteMsg::RemoveEquipment { id } => execute_remove_equipment(deps, id),
        ExecuteMsg::ClearInventory {} => execute_clear_inventory(deps),
        ExecuteMsg::AddUser { email, role } => execute_add_user(deps, env, email, role),
        ExecuteMsg::RemoveWorker { email } => execute_remove_worker(deps, email),
        ExecuteMsg::ResetMaintenance { equipment_id } => {
            execute_reset_maintenance(deps, equipment_id)
        }
        ExecuteMsg::ListEquipment {} => execute_list_equipment(deps),
    }
}

pub fn query(deps: DepsMut, msg: QueryMsg) -> StdResult<Response> {
    match msg {
        QueryMsg::GetEquipmentInfo { equipment_id } => query_equipment_info(deps, equipment_id),
        QueryMsg::GetMaintenanceHistory { equipment_id } => query_maintenance_history(deps, equipment_id),
        QueryMsg::ListEquipment {} => query_list_equipment(deps),
        QueryMsg::GetUserDetails { email } => query_user_details(deps, email),
    }
}
