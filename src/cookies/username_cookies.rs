use gloo::storage::LocalStorage;
use gloo_storage::*;
use serde::{Deserialize,Serialize};
#[derive(Debug,Deserialize,Serialize)]
struct UsernameLocal {
    username:String
}

pub fn get_username_storage() -> Result<String> {
    let username = LocalStorage::get::<UsernameLocal>("username")?;
    Ok(username.username.clone())
}
pub fn set_username_storage(username:&str) -> Result<()> {
    let username = UsernameLocal {
        username:String::from(username)
    };
    LocalStorage::set("username",username)?;
    Ok(())
}
