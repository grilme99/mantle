use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::super::roblox_resource_manager::RobloxResource;

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV4 {
    pub environments: HashMap<String, Vec<RobloxResource>>,
}
