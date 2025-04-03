use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

pub const CONTRACT_VERSION: &str = "crates.io:maintenance-scheduler:v1.0.0";
pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");
pub const EQUIPMENT: Map<&str, Equipment> = Map::new("equipment");
pub const MAINTENANCE_LOGS: Map<&str, Vec<MaintenanceLog>> = Map::new("maintenance_logs");
pub const USERS: Map<&str, User> = Map::new("users");

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
