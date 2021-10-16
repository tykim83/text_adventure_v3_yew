use crate::game_logic::map::{Direction, Location};
use yew::{html, Component, Context, Html, Properties, Callback};

pub enum Msg {
    GoTo(Direction),
}

#[derive(Clone, Properties, Debug, PartialEq)]
pub struct Props {
    pub north: Option<Location>,
    pub south: Option<Location>,
    pub east: Option<Location>,
    pub west: Option<Location>,
    pub on_go_to: Callback<Location>,
}

pub struct Compass {
    north: Option<Location>,
    south: Option<Location>,
    east: Option<Location>,
    west: Option<Location>,
}

impl Component for Compass {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            north : ctx.props().north,
            south : ctx.props().south,
            west : ctx.props().west,
            east : ctx.props().east,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoTo(direction) if direction == Direction::North => {
                ctx.props().on_go_to.emit(self.north.unwrap());
                true
            },
            Msg::GoTo(direction) if direction == Direction::South => {
                ctx.props().on_go_to.emit(self.south.unwrap());
                true
            },
            Msg::GoTo(direction) if direction == Direction::West => {
                ctx.props().on_go_to.emit(self.west.unwrap());
                true
            },
            Msg::GoTo(direction) if direction == Direction::East => {
                ctx.props().on_go_to.emit(self.east.unwrap());
                true
            },
            _ => true,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.north = ctx.props().north;
        self.south = ctx.props().south;
        self.west = ctx.props().west;
        self.east = ctx.props().east;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <div class="row">
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick={ctx.link().callback(|_| Msg::GoTo(Direction::North))}
                                disabled={self.north.is_none()}> { "North" } 
                        </button>
                    </div>
                    <div class="col-4"></div>
                </div>
                <div class="row">
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick={ctx.link().callback(|_| Msg::GoTo(Direction::West))} 
                                disabled={self.west.is_none()}> { "West" }
                        </button>
                    </div>
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick={ctx.link().callback(|_| Msg::GoTo(Direction::East)) }
                                disabled={self.east.is_none()}> { "East" } 
                        </button>
                    </div>
                </div>
                <div class="row">
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                                onclick={ctx.link().callback(|_| Msg::GoTo(Direction::South))}
                                disabled={self.south.is_none()}> { "South" } 
                        </button>
                    </div>
                    <div class="col-4"></div>
                </div>
            </div>
        }
    }
}
