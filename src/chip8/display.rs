use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings, EventLoop};
use glutin_window::GlutinWindow;

pub struct Display {
    pub window: GlutinWindow,
    pub events: Events
}

impl Display {
    pub fn new() -> Display {
        let settings = WindowSettings::new("Rusty Chip 8 Emulator", [512; 2])
                            .exit_on_esc(true);

        Display {
        window: settings.build().expect("Could not build window!"),
        events:  Events::new(EventSettings::new().max_fps(60))
        }
    }

    pub fn update_display(&self, display_memory: Vec<u8>) {
        panic!("method not implemented")
    }

    pub fn clear_display(&self) {
        panic!("method not implemented")
    }
}