use std::sync::Arc;

use async_std::channel::{unbounded, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct Event {
    pub i: u32,
    pub n: Arc<String>,
}

impl Event {
    pub fn new(i: u32, n: Arc<String>) -> Self {
        Self { i, n }
    }
}

#[derive(Clone)]
pub struct EventHandle {
    pub receiver: Receiver<Event>,
    pub sender: Sender<Event>,
}

impl Default for EventHandle {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        EventHandle { receiver, sender }
    }
}

pub fn criss_cross() -> (EventHandle, EventHandle) {
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();
    let a = EventHandle {
        receiver: r1,
        sender: s2,
    };
    let b = EventHandle {
        receiver: r2,
        sender: s1,
    };
    (a, b)
}
