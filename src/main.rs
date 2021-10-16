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

#[derive(Clone, Properties, Debug)]
pub struct Props {
    map: Map,
    current_location: Location,
}


pub struct Index {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: Props {
                map: Map::init(),
                current_location: Location::Kitchen,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoTo(location) => {
                self.props.current_location = location;
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let room = self.props.map.current_location(&self.props.current_location);

        let description_props = yew::props!(Description::Properties {
            room_name: room.name,
            room_description: room.description,
        });

        let compass_props = yew::props!(Compass::Properties {
            north: room.exit.get(&Direction::North).copied(),
            south: room.exit.get(&Direction::South).copied(),
            east: room.exit.get(&Direction::East).copied(),
            west: room.exit.get(&Direction::West).copied(),
            on_go_to: self.link.callback(Msg::GoTo),
        });

        html! {
            <div class="container-fluid p-5">
                <div class="row">
                    <div class="col-8">
                        <Description with description_props />
                    </div>
                    <div class="col-4">
                        <Compass with compass_props />
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