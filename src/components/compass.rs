use crate::game_logic::map::{Direction, Location};
use yew::prelude::*;
use yew::services::ConsoleService;

pub enum Msg {
    GoTo(Direction),
}

#[derive(Clone, Properties, Debug)]
pub struct Props {
    pub north: Option<Location>,
    pub south: Option<Location>,
    pub east: Option<Location>,
    pub west: Option<Location>,
    pub on_go_to: Callback<Location>,
}

pub struct Compass {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Compass {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoTo(direction) if direction == Direction::North => {
                self.props.on_go_to.emit(self.props.north.unwrap());
                true
            },
            Msg::GoTo(direction) if direction == Direction::South => {
                self.props.on_go_to.emit(self.props.south.unwrap());
                true
            },
            Msg::GoTo(direction) if direction == Direction::West => {
                self.props.on_go_to.emit(self.props.west.unwrap());
                true
            },
            Msg::GoTo(direction) if direction == Direction::East => {
                self.props.on_go_to.emit(self.props.east.unwrap());
                true
            },
            _ => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="row">
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick=self.link.callback(|_| Msg::GoTo(Direction::North))
                                disabled=self.props.north.is_none()> { "North" } 
                        </button>
                    </div>
                    <div class="col-4"></div>
                </div>
                <div class="row">
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick=self.link.callback(|_| Msg::GoTo(Direction::West)) 
                                disabled=self.props.west.is_none()> { "West" }
                        </button>
                    </div>
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick=self.link.callback(|_| Msg::GoTo(Direction::East)) 
                                disabled=self.props.east.is_none()> { "East" } 
                        </button>
                    </div>
                </div>
                <div class="row">
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick=self.link.callback(|_| Msg::GoTo(Direction::South))
                                disabled=self.props.south.is_none()> { "South" } 
                        </button>
                    </div>
                    <div class="col-4"></div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
