use yew::prelude::*;

pub struct Number {
    link: ComponentLink<Self>,
    props: NumberProp,
}

#[derive(Debug, Clone, Properties)]
pub struct NumberProp {
    pub num: i64
}

impl Component for Number {
    type Message = ();
    type Properties = NumberProp;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ self.props.num }</p>
            </div>
        }
    }
}

