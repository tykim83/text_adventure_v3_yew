pub mod components;
pub mod game_logic;

use components::compass::Compass;
use components::description::Description;
use game_logic::map::{Direction, Location, Map};
// use yew::services::ConsoleService;
use yew::{html, prelude::*};

pub enum Msg {
    GoTo(Location),
}

pub struct Index {
    map: Map,
    current_location: Location,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            map: Map::init(),
            current_location: Location::Kitchen,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoTo(location) => {
                self.current_location = location;
                true
            },
        }
    }

    // fn changed(&mut self, ctx: &Context<Self>) -> bool {
    //     false
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let room = self.map.current_location(&self.current_location);

        let description_props = yew::props!(Description::Properties {
            room_name: room.name,
            room_description: room.description,
        });

        let _compass_props = yew::props!(Compass::Properties {
            north: room.exit.get(&Direction::North).copied(),
            south: room.exit.get(&Direction::South).copied(),
            east: room.exit.get(&Direction::East).copied(),
            west: room.exit.get(&Direction::West).copied(),
            on_go_to: ctx.link().callback(Msg::GoTo),
        });

        html! {
            <div class="container-fluid p-5">
                <div class="row">
                    <div class="col-8">
                        <Description
                            room_name={description_props.room_name}
                            room_description={description_props.room_description}    
                        />
                    </div>
                    <div class="col-4">
                        <Compass 
                        north={room.exit.get(&Direction::North).copied()}
                        south={room.exit.get(&Direction::South).copied()}
                        east={room.exit.get(&Direction::East).copied()}
                        west={room.exit.get(&Direction::West).copied()}
                        on_go_to={ctx.link().callback(Msg::GoTo)}
                        />
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Index>();
}

// clone gives you Option<T> from Option<T>, cloned gives you Option<T> from Option<&T>
// ConsoleService::info(format!("Update: {:?}", msg).as_ref());