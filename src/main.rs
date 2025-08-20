mod c1;
mod chat;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::hooks::use_navigator;
use log::info;
//use gloo_net::http::Request;
//use crate::c1::*;
use crate::chat::*;
//use wasm_bindgen::JsValue;

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



#[function_component(Game)]
fn game() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Game" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
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
    info!("Home");
    //let token = use_state(|| String::new());
    /*
    {
        let t = token.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get("http://localhost:8000/api/token")
                    .send()
                    .await
                    .unwrap();

                t.set(res.text().await.unwrap());
            });
        });
    }



    let b1 = use_state(|| true);
    let onclick = {
        let b1 = b1.clone();
        Callback::from(move |_| {
            b1.set(false);
        })
    };
     */

        let items = vec![
        QaItem {
            question: "Does the API support rate limiting?".into(),
            verdict: Verdict::Yes,
            explanation: "Requests are throttled to protect stability. Short bursts are fine; sustained high traffic is shaped.".into(),
        },
        QaItem {
            question: "Is offline mode available by default?".into(),
            verdict: Verdict::No,
            explanation: "Requires a local cache or extension; the app expects a live connection by default.".into(),
        },
        QaItem {
            question: "Is water wet?".into(),
            verdict: Verdict::Unable,
            explanation: "Not a useful yes/no in this context. Try asking about behavior or properties instead.".into(),
        },
        QaItem {
            question: "Can I export my data?".into(),
            verdict: Verdict::Other("Maybe".into()),
            explanation: "Export is available for paid plans only; check your subscription.".into(),
        },
    ];
    let on_submit = Callback::from(|text: String| {
        web_sys::console::log_1(&format!("User asked: {}", text).into());
        // You’d typically send this upstream or mutate state here.
    });

    html! {
        <>
        <QaList items={items} on_submit={on_submit}/>
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
