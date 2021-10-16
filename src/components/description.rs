use yew::prelude::*;
// use yew::services::ConsoleService;

pub enum Msg {}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub room_name: String,
    pub room_description: String,
}

pub struct Description {
    room_name: String,
    room_description: String,
}

impl Component for Description {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            room_name: ctx.props().room_name.clone(),
            room_description: ctx.props().room_description.clone(),
         }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.room_name = ctx.props().room_name.clone();
        self.room_description = ctx.props().room_description.clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>{ &self.room_name }</div>
                <div>{ &self.room_description }</div>
            </>
        }
    }
}
