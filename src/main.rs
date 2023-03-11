use yew::prelude::*;
use yew_router::prelude::*;
mod requests;
mod pages;
mod input;
mod cookies;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html!{
        <>
            <BrowserRouter>
                <Switch<pages::routers::Route> render={pages::routers::switch}/>
            </BrowserRouter>
        </>
    }
}

