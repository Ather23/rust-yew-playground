use crate::modules::navbar::NavBar;
use crate::Route;
use yew::{html, Component, Context, Html, Properties};
use yew_router::prelude::Link;
pub struct HomePage;

#[derive(Clone, PartialEq, Properties)]
pub struct HomePageProps {}

impl Component for HomePage {
    type Properties = HomePageProps;
    type Message = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let page = ctx.props().to_owned().page;
        html! {
          <>
            <NavBar/>
            <h1>{"Home"}</h1>
            <Link<Route> to={Route::Game}>
                <button
                    class="px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                    {"Go Chatting!"}
                </button>
            </Link<Route>>
          </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
}
