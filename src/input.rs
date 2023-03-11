use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement,EventTarget};
pub fn input(join_input:UseStateHandle<String>) -> Html
{
    let value = format!("{}",join_input.as_str());
    let input = move |e:Event| {
        let join_code = &join_input;
        let target:Option<EventTarget> = e.target();
        let html_input:HtmlInputElement = target
            .expect("error getting input")
            .dyn_into()
            .unwrap();
        join_code.set(html_input.value());
    };
    html!{
        <>
            <input type="text" value={value} onchange={input}/>
        </>
    }
}
