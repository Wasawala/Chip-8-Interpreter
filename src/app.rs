use sdl3::{
    Sdl, VideoSubsystem,
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    render::Canvas,
    video::Window,
};
use std::time::Duration;

use crate::chip8::Interpreter;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub struct WindowSpecs {
    name: String,
    width: u32,
    height: u32,
    centered: bool,
    fullscreen: bool,
}

impl WindowSpecs {
    pub fn new(name: String, width: u32, height: u32, centered: bool, fullscreen: bool) -> Self {
        Self {
            name,
            width,
            height,
            centered,
            fullscreen,
        }
    }
}
pub struct App {
    sdl_context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<Window>,
    interpreter: Interpreter,
}

impl App {
    pub fn new(specs: WindowSpecs, rom_path: &str) -> Self {
        let context = sdl3::init().unwrap();
        let video = context.video().unwrap();
        let canvas = Self::create_window(&video, specs);

        let interpreter = Interpreter::new(rom_path);
        Self {
            sdl_context: context,
            video,
            canvas,
            interpreter,
        }
    }

    fn create_window(video: &VideoSubsystem, specs: WindowSpecs) -> Canvas<Window> {
        let mut window_builder = video.window(&specs.name, specs.width, specs.height);
        if specs.centered {
            window_builder.position_centered();
        }
        if specs.fullscreen {
            window_builder.fullscreen();
        }

        let window = window_builder.build().unwrap();
        window.into_canvas()
    }

    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();

        let texture_creator = self.canvas.texture_creator();

        let texture = &mut texture_creator
            .create_texture_streaming(PixelFormat::RGB24, WIDTH, HEIGHT)
            .unwrap();

        self.interpreter.dump_memory();

        let mut last_time = sdl3::timer::performance_counter();
        let freq = sdl3::timer::performance_frequency() as f64;

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            let current_time = sdl3::timer::performance_counter();
            let delta_time = (current_time - last_time) as f64 / freq;

            if delta_time > 1.0 / 60.0 {
                last_time = current_time;
                self.interpreter.dec_timers();
            }

            self.canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    _ => {}
                }
            }

            self.interpreter
                .next_instruction(event_pump.keyboard_state());

            self.interpreter.draw(&mut self.canvas, texture);
            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
        }
    }
}
