use crossterm::event::{Event, poll, read};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn poll(&self) -> Option<Event> {
        if poll(std::time::Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            Some(event)
        } else {
            None
        }
    }
}
