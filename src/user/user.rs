// UserCookies is a simple login cookie implements

use uuid::Uuid;

pub struct UserCookies {
    // cookie -> username
    uuid_username: std::collections::HashMap<String, String>,
}

impl UserCookies {
    pub fn new() -> Self {
        UserCookies {
            uuid_username: std::collections::HashMap::new(),
        }
    }

    pub fn create(&mut self, username: &str) -> String {
        // TODO: TTL
        let id = Uuid::new_v4().to_string();
        self.uuid_username.insert(id.clone(), username.to_string());
        id
    }

    pub fn get_username(&self, id: &str) -> Option<&String> {
        self.uuid_username.get(id)
    }
}
