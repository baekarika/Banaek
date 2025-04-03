use cosmwasm_std::{DepsMut, Response, StdResult};
use crate::state::{EQUIPMENT, USERS, MAINTENANCE_LOGS, Equipment, User, UserRole};

pub fn execute_add_equipment(
    deps: DepsMut,
    id: String,
    name: String,
    description: String,
    usage_threshold: u64,
) -> StdResult<Response> {
    let equipment = Equipment {
        id: id.clone(),
        name,
        description,
        usage_threshold,
        last_maintenance: 0,
        total_usage: 0,
    };

    EQUIPMENT.save(deps.storage, &id, &equipment)?;

    Ok(Response::new().add_attribute("action", "add_equipment"))
}

pub fn execute_remove_equipment(deps: DepsMut, id: String) -> StdResult<Response> {
    EQUIPMENT.remove(deps.storage, &id);
    MAINTENANCE_LOGS.remove(deps.storage, &id);

    Ok(Response::new().add_attribute("action", "remove_equipment"))
}
