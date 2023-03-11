use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo::console::log;
use crate::{requests,input,pages::routers};
use yew_router::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    let join_code =  use_state(|| {
        String::from("join code")
    });
    let name =  use_state(|| {
        String::from("name")
    });

    let code =  use_state(|| {
        String::new()
    });
    let join_result = use_state(|| {
        String::new()
    });
    let navigator = use_navigator().unwrap();
    let j = join_code.clone();
    let n = name.clone();
    let c = code.clone();
    let nav = navigator.clone();
    let jr = join_result.clone();
    let submit = move |_| {
        let j = j.to_string();
        let jr = jr.clone();
        let n = nav.clone();
        spawn_local(async move {
            if requests::database::is_valid_chatroom(&j).await {
                jr.set(String::from("Joining ... "));
                log!("{}",&j);
                n.push(&routers::Route::Pages { join_code: j });
            } else {
                jr.set(String::from("No server with that join code"));
                log!("invalid");
            }
        });
    };
    let create = move |_| {
        log!("sending request");
        let n = n.to_string();
        let c = c.clone();
        spawn_local(async move {
           let join_code = requests::database::new_chatroom(n).await.unwrap(); 
           c.set(join_code);
        });
    };
    html!{
        <>
            <h1>{"Welcome to Chatr"}</h1>
            <h1>{join_result.to_string()}</h1>
            <label>{"Enter join code: "}</label>
            {input::input(join_code.clone())}
            <button onclick={submit}>{"join"}</button>
            <div>
                <h1>{code.to_string()}</h1>
                <label>{"Enter name: "}</label>
                {input::input(name.clone())}
                <button onclick={create}>{"create"}</button>
            </div>
       </>
    }
}
