use yew::{html, Component, Context, Html, Properties};
pub struct GamePage;

#[derive(Clone, PartialEq, Properties)]
pub struct GameProps {}

impl Component for GamePage {
    type Properties = GameProps;
    type Message = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"Game"}</h1>
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
