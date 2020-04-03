extern crate sdl2;
use emulator::cpu::CPU;
use emulator::Emulator;
use std::time::{Duration, Instant};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
// use std::thread;
mod emulator;

const SCALE_FACTOR: i32 = 2;


fn main() {
    let mut cpu = CPU::new();
    let mut emulator = Emulator::new();
    emulator.load_rom(&mut cpu, 0x00);
    
    let sdl_context = sdl2::init().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Intel 8080 Emulator", (224 * SCALE_FACTOR) as u32, (256 * SCALE_FACTOR) as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
	// let audio_device = audio::initialize(&sdl_context)?;



    'running: loop {
        //TODO: Implement Interrupts
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        let instant = Instant::now();
        if cpu.last_timer == 0 {
            cpu.last_timer = instant.elapsed().as_micros();
            // cpu.next_interrupt = cpu.last_timer + 16000;
            // cpu.which_interrupt = 1;
        }

        if instant.elapsed().as_micros().wrapping_sub(cpu.last_timer) > 1600 {
            //Redraw Screen
            redraw_screen(&mut canvas, &mut cpu);
        }

        if (cpu.interrupts_enabled) && (instant.elapsed().as_micros() > cpu.next_interrupt) {
            //Call Interrupts
        }

        let since_last = instant.elapsed().as_micros().wrapping_sub(cpu.last_timer);
        let cycles_to_catch:u128 = since_last.wrapping_mul(2);
        let mut cycles:u128 = 0;
        
        while cycles_to_catch > cycles {
            println!("TO CATCH: {}, DONE: {}",since_last,cycles);
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            cycles += emulator.emulate(&mut cpu);
        }
        // cpu.last_timer = instant.elapsed().as_micros(); //Seems Wrong
    }
    
}

fn redraw_screen(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,cpu: &mut CPU) {
    let width:u32 = 224;
    let height:u32 = 256;
    let mut i:usize = 0x2400;
    let mut col = 0;
    while col < width {
        let mut row = height;
        while row > 0 {
            let mut j = 0;
            while j<8 {
                let idx = (row-j)*width+col;
                if (cpu.memory[i] & 1 << j) != 0 {
                    let x = (idx % width) as i32;
                    let mut temp = idx as f32/width as f32;
                    temp = temp.floor();
                    let y = temp as i32;
                    draw_pixel(canvas, x, y);
                }
                j += 1;
            }
            i += 1;
            row -= 8;
        }
        col += 1;
    }
}

fn draw_pixel(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, ) {
    let width = 224*SCALE_FACTOR;
    let height = 256*SCALE_FACTOR;
    let newx = x*width/224;
    let newy = y*height/256;
    let pixel_width = ((x + 1) * width / 224) - newx;
    let pixel_height = ((y + 1) * height / 256) - newy;
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.fill_rect(Rect::new(newx, newy, pixel_width as u32, pixel_height as u32));
    canvas.present();
}