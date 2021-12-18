use std::collections::VecDeque;

use yew::prelude::*;
use yew_feather::edit::Edit;
use yew_feather::x_circle::XCircle;

#[derive(Clone)]
pub struct TodoData {
    pub id: usize,
    pub text: String,
    pub is_complete: bool
}

impl TodoData {
    pub fn is_complete(&self) -> bool {
        false
    }

    pub fn toggle_complete(&mut self) {
        self.is_complete = !self.is_complete;
    }
}

pub struct Todo {
    link: ComponentLink<Self>,
    edit: String,
    props: TodoProps
}

#[derive(Clone, Properties)]
pub struct TodoProps {
    pub todos: VecDeque<TodoData>,
}

pub enum TodoMessage {
    CompleteTodo(usize)
}

impl Component for Todo {
    type Message = TodoMessage;
    type Properties = TodoProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            edit: String::new(),
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoMessage::CompleteTodo(id) => {
                let mut updated_todos = VecDeque::new();
                for todo in self.props.todos.iter() {
                    updated_todos.push_back(todo.clone());
                    let last = updated_todos.back_mut().unwrap();
                    if last.id == id {
                        last.toggle_complete();
                    }
                }

                true
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut html_list = Vec::new();
        for (i, todo) in self.props.todos.iter().enumerate() {
            let id: usize = todo.id;
            html_list.push(html! {
                <div class=classes!(if todo.is_complete() { "todo-row complete" } else { "todo-row" }) key={i}>
                    <div key={id} onclick={self.link.callback(move |_| TodoMessage::CompleteTodo(id))}>
                        { todo.text.clone().as_str() }
                    </div>
                    <div>
                        <XCircle />
                        <Edit />
                    </div>
                </div>
            });
        }

        html! {
            {
                html_list.into_iter().map(|todo_item| {
                    todo_item
                }).collect::<Html>()
            }
        }
    }
}

