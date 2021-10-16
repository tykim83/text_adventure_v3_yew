use yew::prelude::*;
// use yew::services::ConsoleService;

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    pub room_name: String,
    pub room_description: String,
}

pub struct Description {
    props: Props,
    _link: ComponentLink<Self>,
}

impl Component for Description {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props, _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>{ &self.props.room_name }</div>
                <div>{ &self.props.room_description }</div>
            </>
        }
    }
}
