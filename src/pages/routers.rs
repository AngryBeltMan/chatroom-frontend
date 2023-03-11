use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{chatroom,home}; // all of the pages of the website
#[derive(Debug,Clone,PartialEq,Routable)]
pub enum Route{ 
    #[at("/")]
    Home,
    #[at("/chatroom/:join_code")]
    Pages {join_code:String},
}
pub fn switch(route:Route) -> Html {
    match route {
        Route::Home => return html! { <home::Home/> },
        Route::Pages {join_code} => return html! { <chatroom::Chatroom value={join_code}/> },
    }
}
