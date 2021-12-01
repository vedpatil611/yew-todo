use std::rc::Rc;
use std::cell::RefCell;

use yew::prelude::*;
use crate::components::todo_form::TodoForm;

#[derive(Clone)]
pub struct TodoModal {
    pub text: String
}

pub struct TodoList {
    _link: ComponentLink<Self>,
    todos: Rc<RefCell<Vec<TodoModal>>>
}

pub enum TodoListMsg {
    SubmitTodo
}

impl Component for TodoList {
    type Message = TodoListMsg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            _link,
            todos: Rc::new(RefCell::new(Vec::new()))
        }
    }
    
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Plan for today"}</h1>
                <TodoForm todos={Rc::clone(&self.todos)}/>
            </div>
        }
    }
}
