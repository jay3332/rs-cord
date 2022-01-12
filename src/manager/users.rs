use crate::{ClientState, User};
use crate::types::user::UserData;

use std::collections::HashMap;

#[derive(Debug)]
pub struct UserManager {
    state: ClientState,
    
    /// The actual internal cache being used.
    /// 
    /// This is a HashMap of snowflake IDs to User models.
    pub cache: HashMap<u64, User>,
}

impl UserManager {
    pub(crate) fn new(state: ClientState) -> Self {
        Self {
            state,
            cache: HashMap::new(),
        }
    }
    
    /// Returns a User model for the given ID, if applicable.
    pub fn get(&self, id: u64) -> Option<&User> {
        self.cache.get(&id)
    }

    /// Given [`UserData`][`crate::types::user::UserData`], create a new user and add it into the cache.
    /// 
    /// If the user already exists in the cache, this will call `User::patch_from_user_data` (private).
    pub fn add_from_data(&mut self, data: UserData) {
        let id: u64 = data.id.parse().unwrap();

        match self.get(id).as_deref_mut() {
            Some(user) => {
                user.patch_from_user_data(data);
            },
            None => {
                let user = User::from_user_data(self.state.clone(), data);
                self.cache.insert(id, user);
            },
        }
    }

    /// Overwrites the user in the cache with this user.
    /// 
    /// Unlike [`add_from_data()`][`UserManager::add_from_data`], this takes a User model instead of User data.
    pub fn overwrite_user(&mut self, user: User) {
        self.cache.insert(user.id, user);
    }

    pub async fn fetch(&self, id: u64) -> crate::ThreadSafeResult<&User> {
        unimplemented!(); // TODO
    }
}
