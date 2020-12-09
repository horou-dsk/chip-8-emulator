// 设置窗口显示，隐藏控制台
#![windows_subsystem = "windows"]

use chip_8_emulator::cpu::Cpu;
use chrono::Local;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::Duration;
use std::thread::sleep;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::Read;

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    // let rom_file = File::open("roms/UFO");
    let rom_file = File::open("roms/TETRIS");
    // let rom_file = File::open("roms/PONG");
    let mut rom_data = Vec::new();
    match rom_file {
        Ok(mut file) => {
            file.read_to_end(&mut rom_data).unwrap();
        }
        Err(_) => {
            panic!("rom load failed!");
        }
    }
    let mut cpu = Cpu::new(rom_data);
    let mut next_game_tick = Local::now().timestamp_nanos() as f64;
    'running: loop {
        next_game_tick += MS_PER_UPDATE;
        let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(16, 29, 43));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    cpu.keys[0] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    cpu.keys[1] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    cpu.keys[2] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    cpu.keys[3] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    cpu.keys[4] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    cpu.keys[5] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    cpu.keys[6] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cpu.keys[7] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    cpu.keys[8] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    cpu.keys[9] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    cpu.keys[10] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    cpu.keys[11] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    cpu.keys[12] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    cpu.keys[13] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    cpu.keys[14] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    cpu.keys[15] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                    cpu.keys[0] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
                    cpu.keys[1] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
                    cpu.keys[2] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {
                    cpu.keys[3] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                    cpu.keys[4] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    cpu.keys[5] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                    cpu.keys[6] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                    cpu.keys[7] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    cpu.keys[8] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    cpu.keys[9] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    cpu.keys[10] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::F), .. } => {
                    cpu.keys[11] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                    cpu.keys[12] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                    cpu.keys[13] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                    cpu.keys[14] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::V), .. } => {
                    cpu.keys[15] = false;
                },
                _ => {
                    // println!("{:?}", e);
                }
            }
        }
        // The rest of the game loop goes here...

        for _ in 0..10 {
            cpu.step();
        }
        canvas.set_draw_color(Color::RGB(0x8F, 0x91, 0x86));
        // let mut gfx = [0u8; 2048];
        // draw
        for i in 0..2048 {
            // println!("x = {}, y = {}", i % 64, i / 64);
            let px = cpu.gfx[i];
            if px != 0 {
                let rect = Rect::new((i % 64 * 10) as i32, (i / 64 * 10) as i32, 10, 10);
                canvas.fill_rect(rect).unwrap();
                canvas.draw_rect(rect).unwrap();
            }
        }

        if cpu.step_num % 2 == 0 {
            cpu.update_timers();
        }
        cpu.step_num += 1;
        canvas.present();
        if sleep_time > 0.0 {
            sleep(Duration::new(0, sleep_time as u32));
        }
    }
}
