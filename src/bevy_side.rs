use weblog::console_log;

use bevy::app::Events;
use bevy::prelude::{self, *};

use crate::common::{Event, EventHandle};

pub struct Plugin {
    pub handle: EventHandle,
}

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(self.handle.clone())
            .add_event::<Send>()
            .add_event::<Receive>()
            .add_system(send.system())
            .add_system(receive.system())
            .add_system(log.system());
    }
}

#[derive(Debug)]
pub struct Send(Event);

#[derive(Debug)]
pub struct Receive(Event);

fn receive(handle: ResMut<EventHandle>, mut events: ResMut<Events<Receive>>) {
    if let Ok(ev) = handle.receiver.try_recv() {
        log::info!("bevy receiving event {:?}", ev);
        events.send(Receive(ev));
    }
}

fn send(handle: ResMut<EventHandle>, mut reader: EventReader<Send>) {
    for ev in reader.iter() {
        if let Err(_e) = handle.sender.try_send(ev.0.clone()) {
            console_log!("bevy Error sending event");
        }
    }
}

fn log(mut reader: EventReader<Receive>) {
    for ev in reader.iter() {
        console_log!("bevy receiving Event: {:?}", ev.0.i);
    }
}
