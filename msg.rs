use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum UserRole {
    Engineer,
    Supervisor,
    Admin,
}
