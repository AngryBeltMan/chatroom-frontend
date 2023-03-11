use gloo::console::log;
use yew_hooks::{use_async,use_interval};
use getrandom::getrandom;
use yew::prelude::*;
use crate::{pages::PropString,requests,input,cookies::username_cookies};
use wasm_bindgen_futures::spawn_local;
use stylist::Style;
use std::sync::Arc;
const MESSAGESCROLL:&str = include_str!("../css/scroll-messages.css");
// TODO reduce the amount of cloning
#[function_component(Chatroom)]
pub fn chatroom(PropString {value}: &PropString) -> Html {
    let join_code = Arc::new(value.clone());
    let  input_username = use_state(|| String::new());
    let j = Arc::clone(&join_code);
    log!(j.as_str());
    let name = use_async(async move {
        let j = &j;
        if let Ok(o) = requests::database::get_name(j).await {
            return Ok(o);
        } else {
            return Err(String::new())
        }
    });
    let input_message = use_state(|| {
        if let Ok(o) = username_cookies::get_username_storage() {
            return o;
        } 
        gen_anon_username()
    });
    let (m,u) = (input_message.clone(),input_username.clone());
    let j = Arc::clone(&join_code);
    let jcode = join_code.clone();
    let post_message = move |_| {
        let (m,u) = (m.to_string(),u.to_string());
        if (m.len() < 99) && (m.len() > 3) && (u.len() < 99) && (u.len() < 99) && (u.len() > 3) {
            username_cookies::set_username_storage(m.as_str()).unwrap();
            let j = Arc::clone(&j);
            spawn_local(async move {
                if requests::database::is_valid_chatroom(&j).await {
                    requests::database::post_message(u, m, j.as_str()).await.unwrap(); // TODO better error handling
                }
            });
        }
    };
    let messages = use_async(async move {
        display_messages(&jcode).await
    });
    let m = messages.clone();
    use_interval(move || {
        let m = &m;
        m.run();
    }, 15000);
    if !(messages.loading) && *(&messages.data.is_none()) { // prevents sending multiple requests andto the same url and crashing the browser and doing a dos attack to my api
        messages.run();
    }
    if !(name.loading) && *(&name.data.is_none()) { // prevents sending multiple requests andto the same url and crashing the browser and doing a dos attack to my api
        name.run();
    }
    let style = Style::new(MESSAGESCROLL).unwrap(); // unwrap okay because the css is valid
    html! {
        <>
            { if let Some(o) = &name.data {
                    html!{ <h1>{format!("{}",o.as_str())}</h1> }
                } else {
                    html!(<h1>{"loading messages ..."}</h1>)
            } }
            {
                if let Some(o) = &messages.data {
                    html!{
                        <div class={style.clone()}>
                            <ul>
                                {o.clone()}
                            </ul>
                        </div>
                    }
                } else {
                    html!(<h1>{"loading messages ..."}</h1>)
                }
            }
            <h13>{"username: "}</h13>
            { input::input(input_message) }
            <div>
                <h13>{"message: ‎"}</h13> 
                { input::input(input_username) }
            </div>
            <button onclick={post_message}>{"post"}</button>
    </>
    }
}
async fn display_messages(join_code:&str) -> Result<Html,()> {
    log!("fetching");
    let mes = requests::database::get_messages(join_code).await.into_iter().map(|f| {
        html!{
            <div>
                <table>
                    <tr><th>{format!("@{}",f[0])}</th></tr>
                    <tr><td>{format!("{}",f[1])}</td></tr>
                </table>
                <hr/>
            </div>
        }
    }).collect::<Html>();
    Ok(mes)
}
fn gen_anon_username() -> String {
    let num = &mut [0u8;8];
    getrandom(num).unwrap();
    let sum = num.iter().map(|f| *f as u32).sum::<u32>();
    format!("anonymous#{}",sum)
}
