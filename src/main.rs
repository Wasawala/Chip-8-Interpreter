use crate::app::{App, WindowSpecs};

use clap::{Arg, Command};

extern crate sdl3;

mod app;
mod chip8;

fn main() {
    let args = Command::new("CHIP-8 Interpreter")
        .about("Basic command line CHIP-8 Interpreter")
        .arg(Arg::new("file").short('f').required(true))
        .get_matches();

    let filepath = args.get_one::<String>("file").expect("required");
    println!("-f: {:?}", filepath);

    let specs: WindowSpecs = WindowSpecs::new("Title".to_string(), 640, 320, true, false);
    let mut app: App = App::new(specs, filepath);

    app.run();
}
