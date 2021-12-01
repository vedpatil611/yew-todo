use std::rc::Rc; 
use std::cell::RefCell;

use yew::prelude::*;

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
    pub todos: Rc<RefCell<Vec<TodoData>>>,
    //pub complete_todo: fn(usize)
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
                let old_todos_ref = Rc::clone(&self.props.todos);

                let mut updated_todos: Vec<TodoData> = Vec::new();
                for todo in old_todos_ref.borrow().iter() {
                    updated_todos.push(todo.clone());
                    if updated_todos.last().unwrap().id == id {
                        updated_todos.last_mut().unwrap().toggle_complete();
                    }
                }

                old_todos_ref.swap(&RefCell::new(updated_todos));
                false
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let todo_ref = Rc::clone(&self.props.todos);
 
        let mut html_list = Vec::new();
        for (i, todo) in todo_ref.borrow().iter().enumerate() {
            let id: usize = todo.id;
            html_list.push(html! {
                <div class=classes!(if todo.is_complete() { "todo-row complete" } else { "todo-row" }) key={i}>
                    <div key={id} onclick={self.link.callback(move |_| TodoMessage::CompleteTodo(id))}>
                        { todo.text.clone().as_str() }
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

