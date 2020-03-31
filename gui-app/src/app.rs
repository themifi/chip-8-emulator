use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2::{ Sdl, render::WindowCanvas, rect::Rect };
use chip_8_emulator::VM;
use std::fs;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

const PIXEL_SIZE: usize = 10;

pub struct App {
    vm: VM,
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl App {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let vm = VM::new();

        Self {
            vm,
            sdl_context,
            canvas,
        }
    }

    pub fn load_program(&mut self, program_path: &str) {
        let program = fs::read(program_path).unwrap();
        self.vm.load_program(&program);
    }

    pub fn run(&mut self) {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            self.canvas.set_draw_color(BLACK);
            self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            self.vm.exec_current_instruction();

            self.canvas.set_draw_color(WHITE);
            for row in 0..chip_8_emulator::graphics::DISPLAY_ROWS {
                for col in 0..chip_8_emulator::graphics::DISPLAY_COLS {
                    if (self.vm.graphics.display[row] & (1 << col)) != 0 {
                        let pixel = Rect::new((col*PIXEL_SIZE) as i32, (row*PIXEL_SIZE) as i32, PIXEL_SIZE as u32, PIXEL_SIZE as u32);
                        self.canvas.fill_rect(pixel).unwrap();
                    }
                }
            }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        };
    }
}
