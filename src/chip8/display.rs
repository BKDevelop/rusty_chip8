use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings, EventLoop};
use glutin_window::GlutinWindow;

pub struct Display {}

impl Display {
    pub fn new() -> Display {
        let settings = WindowSettings::new("Rusty Chip 8 Emulator", [512; 2])
            .exit_on_esc(true);
        let mut window: GlutinWindow = settings.build().expect("Could not build window!");

        let mut events = Events::new(EventSettings::new().max_fps(60));
        while let Some(e) = events.next(&mut window) {}

        println!("{}", settings.get_exit_on_esc());
        Display {}
    }

    pub fn update_display(&self, display_memory: Vec<u8>) {}

    pub fn clear_display(&self) {}
}