use crate::components::todo_list::TodoList;
use yew::prelude::*;

pub struct Home {
    link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <TodoList />
            </div>
        }
    }
}

