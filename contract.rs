use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::{set_contract_version, CanonicalAddr};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Version info for migration info
const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");
const EQUIPMENT: Map<&str, Equipment> = Map::new("equipment");
const MAINTENANCE_LOGS: Map<&str, Vec<MaintenanceLog>> = Map::new("maintenance_logs");
const USERS: Map<&str, User> = Map::new("users");
const ORGANIZATION_DATA: Map<String, Organization> = Map::new("organization_data");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub version: String,
    pub creator: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Equipment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub usage_threshold: u64,
    pub last_maintenance: u64,
    pub total_usage: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MaintenanceLog {
    pub equipment_id: String,
    pub performed_by: String,
    pub supervisor_approved: bool,
    pub timestamp: u64,
    pub maintenance_type: String,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    pub email: String,
    pub role: UserRole,
    pub organization_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum UserRole {
    Engineer,
    Supervisor,
    Admin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub admin: String,
    pub gas_balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddEquipment {
        id: String,
        name: String,
        description: String,
        usage_threshold: u64,
    },
    LogMaintenance {
        equipment_id: String,
        maintenance_type: String,
        notes: String,
    },
    UpdateThreshold {
        equipment_id: String,
        new_threshold: u64,
    },
    RemoveEquipment {
        id: String,
    },
    ClearInventory {},
    AddUser {
        email: String,
        role: UserRole,
    },
    RemoveWorker {
        email: String,
    },
    ResetMaintenance {
        equipment_id: String,
    },
    ListEquipment {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetEquipmentInfo {
        equipment_id: String,
    },
    GetMaintenanceHistory {
        equipment_id: String,
    },
    ListEquipment {},
    GetUserDetails {
        email: String,
    },
}

pub const CONTRACT_VERSION: &str = "crates.io:maintenance-scheduler:v1.0.0";
pub const CONTRACT_NAME: &str = "maintenance_scheduler";

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_VERSION)?;
    
    // Initialize contract info
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
) -> Result<Response, ContractError> {
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

fn execute_add_equipment(
    deps: DepsMut,
    id: String,
    name: String,
    description: String,
    usage_threshold: u64,
) -> Result<Response, ContractError> {
    let equipment = Equipment {
        id: id.clone(),
        name,
        description,
        usage_threshold,
        last_maintenance: 0,
        total_usage: 0,
    };
    
    EQUIPMENT.save(deps.storage, &id, &equipment)?;
    
    Ok(Response::new()
        .add_event(Event::new("equipment_added")
            .add_attribute("id", &id)))
}

fn execute_log_maintenance(
    deps: DepsMut,
    env: Env,
    equipment_id: String,
    maintenance_type: String,
    notes: String,
) -> Result<Response, ContractError> {
    let user = USERS.load(deps.storage, &env.message.sender.to_string())?;
    
    if matches!(user.role, UserRole::Engineer) {
        let mut log = MaintenanceLog {
            equipment_id: equipment_id.clone(),
            performed_by: env.message.sender.to_string(),
            supervisor_approved: false,
            timestamp: env.block.time.seconds(),
            maintenance_type,
            notes,
        };
        
        MAINTENANCE_LOGS.update(deps.storage, &equipment_id, |logs| {
            logs.push(log.clone());
            Ok(logs)
        })?;
        
        Ok(Response::default()
            .add_event(Event::new("maintenance_pending")
                .add_attribute("equipment_id", &equipment_id)))
    } else {
        Err(ContractError::Unauthorized {})
    }
}
fn execute_update_threshold(
    deps: DepsMut,
    equipment_id: String,
    new_threshold: u64,
) -> Result<Response, ContractError> {
    let mut equipment = EQUIPMENT.load(deps.storage, &equipment_id)?;
    equipment.usage_threshold = new_threshold;
    EQUIPMENT.save(deps.storage, &equipment_id, &equipment)?;
    
    Ok(Response::new()
        .add_event(Event::new("threshold_updated")
            .add_attribute("equipment_id", &equipment_id)
            .add_attribute("new_threshold", &new_threshold.to_string())))
}

fn execute_remove_equipment(
    deps: DepsMut,
    id: String,
) -> Result<Response, ContractError> {
    let user = USERS.load(deps.storage, &deps.api.canonical_address(&deps.env.message.sender)?)?;
    
    if matches!(user.role, UserRole::Admin) {
        EQUIPMENT.remove(deps.storage, &id);
        MAINTENANCE_LOGS.remove(deps.storage, &id);
        
        Ok(Response::new()
            .add_event(Event::new("equipment_removed")
                .add_attribute("id", &id)))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn execute_clear_inventory(
    deps: DepsMut,
) -> Result<Response, ContractError> {
    let user = USERS.load(deps.storage, &deps.api.canonical_address(&deps.env.message.sender)?)?;
    
    if matches!(user.role, UserRole::Admin) {
        EQUIPMENT.clear(deps.storage);
        MAINTENANCE_LOGS.clear(deps.storage);
        
        Ok(Response::new()
            .add_event(Event::new("inventory_cleared")))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn execute_add_user(
    deps: DepsMut,
    env: Env,
    email: String,
    role: UserRole,
) -> Result<Response, ContractError> {
    let admin = USERS.load(deps.storage, &deps.api.canonical_address(&env.message.sender)?)?;
    
    if matches!(admin.role, UserRole::Admin) {
        let canonical_addr = deps.api.canonical_address(&env.message.sender)?;
        USERS.save(deps.storage, &canonical_addr.to_string(), &User {
            email,
            role,
            organization_id: admin.organization_id.clone(),
        })?;
        
        Ok(Response::new()
            .add_event(Event::new("user_added")
                .add_attribute("email", &email)
                .add_attribute("role", &role.to_string())))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn execute_remove_worker(
    deps: DepsMut,
    email: String,
) -> Result<Response, ContractError> {
    let admin = USERS.load(deps.storage, &deps.api.canonical_address(&deps.env.message.sender)?)?;
    
    if matches!(admin.role, UserRole::Admin) {
        let canonical_addr = deps.api.canonical_address(&email)?;
        USERS.remove(deps.storage, &canonical_addr.to_string());
        
        Ok(Response::new()
            .add_event(Event::new("worker_removed")
                .add_attribute("email", &email)))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn execute_reset_maintenance(
    deps: DepsMut,
    equipment_id: String,
) -> Result<Response, ContractError> {
    let user = USERS.load(deps.storage, &deps.api.canonical_address(&deps.env.message.sender)?)?;
    
    if matches!(user.role, UserRole::Supervisor) {
        let mut equipment = EQUIPMENT.load(deps.storage, &equipment_id)?;
        equipment.last_maintenance = deps.env.block.time.seconds();
        EQUIPMENT.save(deps.storage, &equipment_id, &equipment)?;
        
        Ok(Response::new()
            .add_event(Event::new("maintenance_reset")
                .add_attribute("equipment_id", &equipment_id)))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn execute_list_equipment(
    deps: Deps,
) -> Result<Response, ContractError> {
    let equipment_iter = EQUIPMENT.range(deps.storage, None, None, cosmwasm_std::Order::Ascending);
    
    let equipment_list: StdResult<Vec<_>> = equipment_iter
        .map(|item| item.map(|(_, v)| v))
        .collect();
    
    Ok(Response::new()
        .set_data(to_binary(&EquipmentListResponse {
            equipment: equipment_list?,
        })?))
}

pub fn query(deps: Deps, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::GetEquipmentInfo { equipment_id } => {
            let equipment = EQUIPMENT.load(deps.storage, &equipment_id)?;
            to_binary(&EquipmentInfoResponse {
                equipment: Some(equipment),
            })
        }
        QueryMsg::GetMaintenanceHistory { equipment_id } => {
            let logs = MAINTENANCE_LOGS.load(deps.storage, &equipment_id)?;
            to_binary(&MaintenanceHistoryResponse {
                logs,
            })
        }
        QueryMsg::ListEquipment {} => {
            let equipment_iter = EQUIPMENT.range(deps.storage, None, None, cosmwasm_std::Order::Ascending);
            let equipment_list: StdResult<Vec<_>> = equipment_iter
                .map(|item| item.map(|(_, v)| v))
                .collect();
            to_binary(&EquipmentListResponse {
                equipment: equipment_list?,
            })
        }
        QueryMsg::GetUserDetails { email } => {
            let canonical_addr = deps.api.canonical_address(&email)?;
            let user = USERS.load(deps.storage, &canonical_addr.to_string())?;
            to_binary(&UserDetailsResponse {
                user: Some(user),
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EquipmentListResponse {
    pub equipment: Vec<Equipment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EquipmentInfoResponse {
    pub equipment: Option<Equipment>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MaintenanceHistoryResponse {
    pub logs: Vec<MaintenanceLog>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserDetailsResponse {
    pub user: Option<User>,
}