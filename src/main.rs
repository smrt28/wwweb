#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod c1;
mod chat;
mod ask_prompt_component;

use gloo::net::http::{Request, Response};
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


struct SoftState {
    n: UseStateHandle<i32>,
    error_message: UseStateHandle<String>,
}


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
    #[at("/error")]
    Error,
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
        Route::Error => html! { <Error /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn Home() -> Html {
    use_state(|| {});
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| wasm_bindgen_futures::spawn_local({
        let navigator = navigator.clone();
        async move {
            if let Ok(token) = fetch_text("/api/new_game").await {
                info!("res: {:?}", token);
                LocalStorage::set("token", &token).unwrap();
                navigator.push(&Route::Game);
            } else {
                navigator.push(&Route::Error);
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
    if res.status() != 200 {
        return Err(anyhow::anyhow!("status[{}]: {}", path, res.status()));
    }
    let text = res.text().await?;
    info!("res: {:?} {:?}", text, res.status());
    Ok(text)
}

async fn send_question(token: &str, text: &str) -> Result<()> {
    info!("{}: asking: {}", token, text);
    Err(anyhow::anyhow!("ask"))
}


#[function_component]
fn Game() -> Html {
    let navigator = use_navigator().unwrap();
    let debug = use_state(|| { String::new() });
    let question_in_air = use_state(|| { false });

    let token: String = match LocalStorage::get("token").ok() {
        Some(token) => token,
        None => {
            navigator.push(&Route::Home);
            return html! {};
        }
    };

    {
        let debug = debug.clone();
        use_effect_with(token.clone(), move |token: &String| {
            let token = token.clone();
            let debug = debug.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_text(&format!("/api/game/{token}")).await {
                    Ok(res) => debug.set(res),
                    Err(e) => log::error!("fetch /api/game failed: {e:?}"),
                }
            });
            || ()
        });
    }

    let on_send = {
        let token = token.clone();
        Callback::from(move |text: String| {
            let token = token.clone();
            wasm_bindgen_futures::spawn_local(async move {
                send_question(&token, &text).await;
            });
        })
    };

    html! {
        <>
        <h1>{ token }</h1>
        <AskPrompt prompt={"Make your guess..."} on_send={on_send}/>
        <hr/>
        <pre>{ debug.to_string() }</pre>
        <pre>{ question_in_air.to_string() }</pre>
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

#[function_component]
fn Error() -> Html {
    html! {
        <h1>{ "Error" }</h1>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
