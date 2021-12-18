use std::collections::VecDeque;

use yew::prelude::*;
use crate::components::todo_form::TodoForm;

use super::todo::Todo;
use super::todo::TodoData;

pub struct TodoList {
    _link: ComponentLink<Self>,
    todos: VecDeque<TodoData>,
}

pub enum TodoListMsg {
    SubmitTodo(TodoData)
}

impl Component for TodoList {
    type Message = TodoListMsg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            _link,
            todos: VecDeque::new(),
        }
    }
    
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoListMsg::SubmitTodo(todo) => {
                self.todos.push_front(todo)
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Plan for today"}</h1>
                <TodoForm />
                <Todo todos={self.todos.clone()} />
            </div>
        }
    }
}
