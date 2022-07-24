use crate::modules::home::HomePage;
use yew::{html, Component, Context, Html, Properties};
pub struct NavBar;

#[derive(Clone, PartialEq, Properties)]
pub struct NavBarProps {}

impl Component for NavBar {
    type Properties = NavBarProps;
    type Message = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          //<>
          <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <a class="navbar-brand" href="#">{"Rizvi"}</a>
            <button class="navbar-toggler" type="button" data-toggle="collapse"
              data-target="#navbarSupportedContent"
                aria-controls="navbarSupportedContent"
                aria-expanded="false" aria-label="Toggle navigation">
              <span class="navbar-toggler-icon"></span>
            </button>

            <div class="collapse navbar-collapse" id="navbarSupportedContent">
              <ul class="navbar-nav mr-auto">
                <li class="nav-item active">
                  <a class="nav-link" href="#">{"Home"}</a>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="#">{"Link"}</a>
                </li>
                <li class="nav-item dropdown">
                  <a class="nav-link dropdown-toggle"
                  href="#" id="navbarDropdown"
                  role="button" data-toggle="dropdown"
                  aria-haspopup="true" aria-expanded="true">
                    {"Dropdown"}
                  </a>
                  <div class="dropdown-menu" aria-labelledby="navbarDropdown">
                    <a class="dropdown-item" href="#">{"Action"}</a>
                    <a class="dropdown-item" href="#">{"Another"}</a>
                    <div class="dropdown-divider"></div>
                    <a class="dropdown-item" href="#">{"Something else"}</a>
                  </div>
                </li>
                <li class="nav-item">
                  <a class="nav-link disabled" href="#">{"Disabled"}</a>
                </li>
              </ul>
            </div>
          </nav>
          // {context}
          // </>
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
