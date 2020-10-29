#[derive(Debug, Clone, PartialEq)]
pub enum DbPrivilege {
    CanRead,
    CanWrite,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub privileges: Vec<DbPrivilege>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: String::default(),
            password: String::default(),
            privileges: Vec::new(),
        }
    }
}

impl User {
    pub fn new(username: &str, password: &str, privileges: Vec<DbPrivilege>) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            privileges,
        }
    }
}
