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
//use wasm_bindgen::JsValue;
use gloo_storage::{LocalStorage, Storage};


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

    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| wasm_bindgen_futures::spawn_local({
        let navigator = navigator.clone();
        async move {
            let res = Request::get("/api/token")
                .send()
                .await
                .unwrap();
            
            let token = res.text().await.unwrap();
            info!("res: {:?}", token);
            LocalStorage::set("token", &token).unwrap();
            navigator.push(&Route::Game);
        }
    }));
    html! {
        <div>
            <h1>{ "Game" }</h1>
            <button {onclick}>{ "New game" }</button>
        </div>
    }
}


#[function_component]
fn Game() -> Html {
    let navigator = use_navigator().unwrap();

    let on_send = {
        let navigator = navigator.clone();
        Callback::from(move |text: String| {
            info!("on_send: {}", text);

            wasm_bindgen_futures::spawn_local({
                let navigator = navigator.clone();
                async move {
                    let res = Request::get("/api/token")
                        .send()
                        .await
                        .unwrap();
                    info!("res: {:?}", res.text().await.unwrap());

                    navigator.push(&Route::Game);
                }

            });
        })
    };

    html! {
        <>
        <AskPrompt prompt={"Make your guess..."} on_send={on_send}/>
        </>
    }
}


#[function_component]
fn App() -> Html {


    let counter = use_state(|| 0);
    info!("counter is: {:?}", counter);
    /*
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1..." }</button>
            <p>{ *counter }</p>
        </div>
    }
     */
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
