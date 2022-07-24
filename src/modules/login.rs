use yew::{html, Component, Context, Html, Properties};
pub struct LoginPage;

#[derive(Clone, PartialEq, Properties)]
pub struct LoginProps {}

impl Component for LoginPage {
    type Properties = LoginProps;
    type Message = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="login-form">
            <form action="/examples/actions/confirmation.php" method="post">
                <h2 class="text-center">{"Log in"}</h2>
                <div class="form-group">
                    <input type="text" class="form-control" placeholder="Username" required=true/>
                </div>
                <div class="form-group">
                    <input type="password" class="form-control" placeholder="Password" required=true/>
                </div>
                <div class="form-group">
                    <button type="submit" class="btn btn-primary btn-block">{"Log in"}</button>
                </div>
                <div class="clearfix">
                    <label class="float-left form-check-label">
                        <input type="checkbox" class="check-box"/>
                            {"Remember me"}
                    </label>
                    <a href="#" class="font-text">{"Forgot Password?"}</a>
                </div>
            </form>
            <p class="text-center"><a href="#">{"Create an Account"}</a></p>
        </div>
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
