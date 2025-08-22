mod c1;
mod chat;
mod ask_prompt_component;

use gloo::net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::hooks::use_navigator;
use log::info;
//use gloo_net::http::Request;
//use crate::c1::*;
//use crate::chat::*;
use crate::ask_prompt_component::*;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::JsValue;
use anyhow::{Context, Result};


struct SoftState {}


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::Static("Bob"))]
    pub name: AttrValue,
}

#[function_component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name}</> }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Game => html! { <Game /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn Home() -> Html {
    use_state(|| {});
    //use_context::<SoftState>();
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| wasm_bindgen_futures::spawn_local({
        let navigator = navigator.clone();
        async move {
            if let Ok(token) = fetch_text("/api/token").await {
                info!("res: {:?}", token);
                LocalStorage::set("token", &token).unwrap();
                navigator.push(&Route::Game);
            } else {
                navigator.push(&Route::NotFound);
            }
        }
    }));
    html! {
        <div>
            <h1>{ "Game" }</h1>
            <button {onclick}>{ "New game" }</button>
        </div>
    }
}


async fn fetch_text(path: &str) -> Result<String> {
    let res = Request::get(path).send().await?;
    let text = res.text().await?;
    info!("res: {:?}", text);
    Ok(text)
}


#[function_component]
fn Game() -> Html {
    let navigator = use_navigator().unwrap();
    let token: String = match LocalStorage::get("token").ok() {
        Some(token) => token,
        None => {
            navigator.push(&Route::Home);
            return html! {};
        }
    };
    let on_send = {
        let navigator = navigator.clone();
        Callback::from(move |text: String| {
            info!("on_send: {}", text);
        })
    };

    html! {
        <>
        <h1>{ token }</h1>
        <AskPrompt prompt={"Make your guess..."} on_send={on_send}/>
        </>
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    info!("counter is: {:?}", counter);
        html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
