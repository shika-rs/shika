#[derive(PartialEq)]
pub enum Permission {
    Read(String),
    Write(String),
    Delete(String)
}

pub struct Client {
    pub permissions: Vec<Permission>
}

#[macro_export]
macro_rules! authorize {
    ($client:expr, [$($permission:expr),*]) => {
        for permission in &[$($permission),*] {
            if !$client.permissions.contains(permission) {
                return Err(anyhow::anyhow!("Unauthorized".to_string()))
            }
        }
    };
}
