use crate::app::{App, WindowSpecs};

extern crate sdl3;

mod app;
mod chip8;


fn main() {
    let specs : WindowSpecs = WindowSpecs::new("Title".to_string(), 640, 480);
    let mut app : App = App::new(specs, true, true);
    
    app.run();
}
