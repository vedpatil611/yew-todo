use std::rc::Rc;
use std::cell::RefCell;

use yew::prelude::*;

use super::todo_list::TodoModal;

pub struct TodoForm {
    link: ComponentLink<Self>,
    input: String,
    props: TodoFormProps
}

pub enum TodoFormMsg {
    TextChange(String),
    SubmitData,
    SubmitNone,
}

#[derive(Clone, Properties)]
pub struct TodoFormProps {
    pub todos: Rc<RefCell<Vec<TodoModal>>>,
}

impl Component for TodoForm {
    type Message = TodoFormMsg;
    type Properties = TodoFormProps;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::new(),
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoFormMsg::TextChange(data) => {
                self.input = data;
                false
            },
            TodoFormMsg::SubmitData => {
                let todos_ref = Rc::clone(&self.props.todos);
                {
                    todos_ref.borrow_mut().push(TodoModal {
                        text: self.input.clone()
                    });
                }
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
            
            //let todos = self.props.todos.borrow_mut();

            //self.props.onsubmit(TodoModal {
                //text: String::from("")
            //});

            Self::Message::SubmitData
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
