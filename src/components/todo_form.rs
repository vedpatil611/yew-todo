use yew::prelude::*;

use crate::components::todo_list::TodoListMsg;

use super::todo::TodoData; 
use super::todo_list::TodoList;

pub struct TodoForm {
    self_link: ComponentLink<Self>,
    link: ComponentLink<TodoList>,
    input: String,
}

pub enum TodoFormMsg {
    TextChange(String),
    SubmitNone,
}

impl Component for TodoForm {
    type Message = TodoFormMsg;
    type Properties = ();
    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let self_link = link.clone();
        let parent_scope = link.get_parent().unwrap();
        Self {
            self_link,
            link: parent_scope.to_owned().downcast::<TodoList>(),
            input: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoFormMsg::TextChange(data) => {
                self.input = data;
                true
            },
            _ => { false },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let data = self.input.clone();
        let handle_submit = self.link.callback(move |e: FocusEvent| {
            e.prevent_default();
            TodoListMsg::SubmitTodo(TodoData::new(data.clone()))
        });

        let handle_change = self.self_link.callback(|e: yew::html::ChangeData| {
            match e {
                ChangeData::Value(data) => { Self::Message::TextChange(data) },
                _ => { Self::Message::SubmitNone }
            }
        });


        html! {
            <form class=classes!("todo-form") onsubmit={handle_submit}>
                <input type="text" placeholder="Add a todo" name={"text"} class=classes!("todo-input") onchange={handle_change}/>
                <button class=classes!("todo-button")>{"Add Todo"}</button>
            </form>
        }
    }
}
