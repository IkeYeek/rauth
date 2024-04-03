use crate::api_error::ApiError;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Role {
    pub(crate) role: String,
}

impl Role {
    /// checking if role a > role b
    pub(crate) fn superior_to(compare: &Role, to: &Role) -> Result<bool, ApiError> {
        let hierarchy = ["root", "super", "user", "visitor"];
        match (
            hierarchy.iter().position(|&r| r == compare.role),
            hierarchy.iter().position(|&r| r == to.role),
        ) {
            (Some(pos_compare_in_hierarchy), Some(pos_to_in_hierarchy)) => {
                Ok(pos_compare_in_hierarchy < pos_to_in_hierarchy)
            }
            _ => Err(ApiError::Internal),
        }
    }
    pub(crate) fn from(s: &str) -> Result<Role, crate::models::role_model::ApiError> {
        match s {
            "root" => Ok(Role {
                role: "root".into(),
            }),
            "super" => Ok(Role {
                role: "super".into(),
            }),
            "user" => Ok(Role {
                role: "user".into(),
            }),
            "visitor" => Ok(Role {
                role: "visitor".into(),
            }),
            _ => Err(ApiError::Role),
        }
    }
}
