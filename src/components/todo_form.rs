use yew::prelude::*;

pub struct TodoForm {
    link: ComponentLink<Self>,
    input: String
}

pub enum TodoFormMsg {
    TextChange(String),
    SubmitData(String),
    SubmitNone,
}

impl Component for TodoForm {
    type Message = TodoFormMsg;
    type Properties = ();
    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoFormMsg::TextChange(data) => {
                self.input = data;
                false
            },
            TodoFormMsg::SubmitData(data) => {
                self.input = data;
                true
            }
            _ => { false },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handle_submit = self.link.callback(|e: FocusEvent| { 
            e.prevent_default();
            Self::Message::SubmitData(String::from(""))
        });

        let handle_change = self.link.callback(|e: yew::html::ChangeData| {
            match e {
                ChangeData::Value(data) => { Self::Message::TextChange(data) },
                _ => { Self::Message::SubmitNone }
            }
        });

        html! {
            <form class=classes!("todo-form") onsubmit={handle_submit}>
                <input type="text" placeholder="Add a todo" value={self.input.clone()} name={"text"} class=classes!("todo-input") onchange={handle_change}/>
                <button class=classes!("todo-button")>{"Add Todo"}</button>
            </form>
        }
    }
}
