use std::sync::Arc;

// use async_std::channel::{self, Sender};
use bevy::prelude::*;
use bevy_webgl2::WebGL2Plugin;

use log::Level;
use yew::prelude::*;
use yewtil::future::LinkFuture;

use common::{Event, EventHandle};

mod bevy_side;
mod common;

pub enum Msg {
    Send(Event),
    Receive(Event),
    Word(Arc<String>),
}

// model
pub struct App {
    props: Props,
    link: ComponentLink<Self>,
    msg: Arc<String>,
}

#[derive(Properties, Clone, Default)]
pub struct Props {
    handle: EventHandle,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let this = Self {
            props,
            link,
            msg: Arc::new("".to_owned()),
        };
        this.listen();

        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Send(e) => {
                weblog::console_log!("yew trying to send Event: {:?}", e.i);
                weblog::console_log!(format!(
                    "receiver count {}",
                    self.props.handle.receiver.receiver_count()
                ));
                self.props.handle.sender.try_send(e).ok();
            }
            Msg::Receive(e) => {
                weblog::console_log!("yew receiving Event: {:?}", e.i);
                self.listen();
            }
            Msg::Word(w) => {
                self.msg = w;
                return true;
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let click_event = Event::new(1337, self.msg.clone());
        html! {
            <main>
                <input
                    value={self.msg.to_string()}
                    oninput={self.link.callback(|e: InputData| Msg::Word(Arc::new(e.value)))}
                />
                <h1>{ self.msg.clone() }</h1>
                <button
                    class="button"
                    onclick={self.link.callback(move |_| Msg::Send(click_event.clone()))}
                >
                    { "send" }
                </button>
            </main>
        }
    }
}

impl App {
    fn listen(&self) {
        let handle = self.props.handle.clone();
        self.link.send_future(async move {
            loop {
                match handle.receiver.recv().await {
                    Ok(msg) => return Msg::Receive(msg),
                    Err(e) => log::error!("Error receiving event: {:?}", e),
                }
            }
        });
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).expect("init logging");

    let (yeh, beh) = common::criss_cross();

    yew::start_app_with_props::<App>(Props { handle: yeh });

    bevy::app::App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WebGL2Plugin)
        .add_plugin(bevy_side::Plugin { handle: beh })
        .run();
}
