use yew::prelude::Properties;
pub mod chatroom;
pub mod routers;
pub mod home;
#[derive(Properties,PartialEq,Clone)]
pub struct PropString {
    pub value:String
}

