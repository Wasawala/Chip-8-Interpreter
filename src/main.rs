use crate::app::{App, WindowSpecs};

mod app;
mod chip8;

use clap::{Arg, Command};

extern crate sdl3;

fn main() {
    let args = Command::new("CHIP-8 Interpreter")
        .about("Basic command line CHIP-8 Interpreter")
        .arg(Arg::new("file").required(true))
        .arg(
            Arg::new("width")
                .default_value("640")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("height")
                .default_value("320")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("fullscreen")
                .default_value("false")
                .value_parser(clap::value_parser!(bool)),
        )
        .get_matches();

    let filepath = args.get_one::<String>("file").expect("required");

    let width = args.get_one::<u32>("width").unwrap();

    let height = args.get_one::<u32>("height").unwrap();

    let fullscreen = args.get_one::<bool>("fullscreen").unwrap();

    let specs: WindowSpecs =
        WindowSpecs::new("Title".to_string(), *width, *height, true, *fullscreen);
    let mut app: App = App::new(specs, filepath);

    app.run();
}
