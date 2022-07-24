#![recursion_limit = "256"]
use crate::modules::login::LoginPage;
use modules::game::GamePage;
use modules::home::HomePage;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};
mod modules;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/game")]
    Game,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn router(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage/>},
        Route::Game => html! {<GamePage/>},
        Route::Login => html! {<LoginPage/>},
        // Route::NavBar => html! {<NavBar page="2"/>},
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div id="login_form">
            <form action="/action_page.php" method="get" id="nameform">
                <label for="fname">{"First name:"}</label>
                <input type="text" id="fname" name="fname"/><br/>
                <label for="lname">{"Last name:"}</label>
                <input type="text" id="lname" name="lname"/>
            </form>
        </div>
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex w-screen h-screen">
                <Switch<Route> render={Switch::render(router)}/>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}

fn main() {
    yew::start_app::<Main>();
}
