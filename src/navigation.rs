use cosmic::iced::{
    event::{
        self,
        wayland::{self, LayerEvent},
        PlatformSpecific,
    },
    keyboard::{key::Named, Event::KeyPressed, Key},
    Event, Subscription,
};

#[derive(Debug, Clone)]
pub enum EventMsg {
    Next,
    Previous,
    Enter,
    Quit,
    Event(cosmic::iced::keyboard::key::Named),
    None,
}

pub fn sub() -> Subscription<EventMsg> {
    cosmic::iced_futures::event::listen_with(|event, status, _| match (event, status) {
        // Make popup disappear when unfocused on Wayland
        (
            Event::PlatformSpecific(PlatformSpecific::Wayland(wayland::Event::Layer(
                LayerEvent::Unfocused,
                ..,
            ))),
            event::Status::Ignored,
        ) => Some(EventMsg::Quit),
        // Pass relevant key events
        (
            Event::Keyboard(KeyPressed {key: Key::Named(named @ 
                (Named::Enter | Named::Escape | Named::ArrowDown | Named::ArrowUp | Named::ArrowLeft | Named::ArrowRight)),
                 ..}),
            event::Status::Ignored,
        ) => 
            Some(EventMsg::Event(named)),
        _ => None,
    })
}
