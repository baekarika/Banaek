use cosmwasm_std::{Deps, StdResult, Binary, to_binary};
use crate::state::{EQUIPMENT, MAINTENANCE_LOGS, USERS};

pub fn query_equipment_info(deps: Deps, equipment_id: String) -> StdResult<Binary> {
    let equipment = EQUIPMENT.load(deps.storage, &equipment_id)?;
    to_binary(&equipment)
}

pub fn query_maintenance_history(deps: Deps, equipment_id: String) -> StdResult<Binary> {
    let logs = MAINTENANCE_LOGS.load(deps.storage, &equipment_id)?;
    to_binary(&logs)
}

pub fn query_list_equipment(deps: Deps) -> StdResult<Binary> {
    let equipment_list: StdResult<Vec<_>> = EQUIPMENT.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.map(|(_, v)| v))
        .collect();
    to_binary(&equipment_list?)
}
