use crate::api_error::ApiError;
use serde::Serialize;

#[derive(PartialEq, Debug, Serialize)]
pub(crate) struct Role {
    pub(crate) id: i32,
    pub(crate) role: String,
}

impl Role {
    /// checking if role a > role b
    pub(crate) fn superior_to(
        a: Role,
        b: Role,
    ) -> Result<bool, ApiError> {
        let hierarchy = ["root", "super", "user", "visitor"];
        match (
            hierarchy.iter().position(|&r| r == b.role),
            hierarchy.iter().position(|&r| r == a.role),
        ) {
            (Some(pos_a_in_hierarchy), Some(pos_b_in_hierarchy)) => {
                Ok(pos_a_in_hierarchy < pos_b_in_hierarchy)
            }
            _ => Err(ApiError::Internal),
        }
    }
}
