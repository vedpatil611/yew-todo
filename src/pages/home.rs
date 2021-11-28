use crate::components::counter::Counter;
use yew::prelude::*;

use crate::app_router::{ Link, AppRoute };

pub enum Msg {
    AddOne,
    SubtractOne,
}

pub struct Home {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            },
            Msg::SubtractOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::SubtractOne)>{ "-1" }</button>
                <Counter>
                    <p>{ self.value }</p>
                </Counter>
                <Link route=AppRoute::Number(self.value) >{"Show number"}</Link>
            </div>
        }
    }
}

