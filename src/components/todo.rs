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

static mut ID_PROVIDER: usize = 0;

impl TodoData {
    pub fn new(text: String) -> TodoData {
        let id = unsafe {
            ID_PROVIDER += 1;
            ID_PROVIDER
        };

        TodoData {
            id,
            text,
            is_complete: false
        }
    }

    pub fn is_complete(&self) -> bool {
        false
    }

    pub fn toggle_complete(&mut self) {
        self.is_complete = !self.is_complete;
    }
}

pub struct Todo {
    link: ComponentLink<Self>,
    _edit: String,
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
            _edit: String::new(),
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
        let _remove_todo = self.link.callback(|e: Event| {
            Self::Message::CompleteTodo(0)
        });

        let mut html_list = Vec::new();
        for (i, todo) in self.props.todos.iter().enumerate() {
            let id: usize = todo.id;
            html_list.push(html! {
                <div class=classes!(if todo.is_complete() { "todo-row complete" } else { "todo-row" }) key={i}>
                    <div key={id} onclick={self.link.callback(move |_| TodoMessage::CompleteTodo(id))}>
                        { todo.text.clone().as_str() }
                    </div>
                    <div class=classes!("icons")>
                        <XCircle size=20/>
                        <Edit size=20/>
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

