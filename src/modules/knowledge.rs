use crate::modules::navbar::NavBar;
use crate::Route;
use gloo::console::log;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use web_sys::InputEvent;
use web_sys::{EventTarget, HtmlInputElement};
use yew::TargetCast;
use yew::{html, Callback, Component, Context,  , Html, MouseEvent, NodeRef, Properties};
use yew_router::prelude::Link;

#[derive(Clone, PartialEq, Properties)]
pub struct KnowledgeProps {
    pub content: Option<String>,
}
pub enum Msg {
    PostText(String),
    DoNothing,
}

pub struct KnowledgePage {
    pub input_ref: NodeRef,
}

impl Component for KnowledgePage {
    type Properties = KnowledgeProps;
    type Message = Msg;

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cur_input = self.input_ref.clone();
        let on_input = ctx.link().callback(move |e: MouseEvent| {
            let name_input_element = cur_input.cast::<HtmlInputElement>().unwrap();
            match name_input_element.value().parse() {
                Ok(val) => {
                    log!(&val);
                    Msg::PostText(val)
                }
                Err(err) => {
                    log::error!("error ocurred parsing value: {}", err);
                    Msg::DoNothing
                }
            }
        });

        html! {
           <>
            <div class="corpus-input">
                <div class="form-group">
                    <textarea ref={self.input_ref.clone()} class="txt-area">
                    </textarea>
                </div>
                <div class="form-group">
                    <button onclick={on_input}
                        class="btn btn-primary btn-block">{"Add to knowledge model"}</button>
                </div>
            </div>
          </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PostText(txt) => {
                log!("This is the text", &txt);
                let input_element = self.input_ref.clone().cast::<HtmlInputElement>().unwrap();
                input_element.set_value(&txt.to_string());
                true
            }
            Msg::DoNothing => {
                log::debug!("Do nothing");
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        log!("Change is coming!");
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        log!("In rendered method..!!");
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Create method");
        Self {
            input_ref: NodeRef::default(),
        }
    }
}
