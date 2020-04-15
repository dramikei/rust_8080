extern crate sdl2;
use emulator::cpu::CPU;
use emulator::Emulator;
use std::time::{Duration, Instant};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::thread;
mod emulator;

const SCALE_FACTOR: i32 = 3;
const CYCLES_PER_FRAME:u64 = 4_000_000 / 60; 


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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::A), .. } => cpu.in_port1 |= 0x20,
                Event::KeyDown {keycode: Some(Keycode::D), .. } => cpu.in_port1 |= 0x40,
                Event::KeyDown {keycode: Some(Keycode::W), .. } => cpu.in_port1 |= 0x10,

                Event::KeyDown {keycode: Some(Keycode::J), .. } => cpu.in_port2 |= 0x20,
                Event::KeyDown {keycode: Some(Keycode::L), .. } => cpu.in_port2 |= 0x40,
                Event::KeyDown {keycode: Some(Keycode::I), .. } => cpu.in_port2 |= 0x10,

                Event::KeyDown {keycode: Some(Keycode::Num1), .. } => cpu.in_port1 |= 0x04,
                Event::KeyDown {keycode: Some(Keycode::Num2), .. } => cpu.in_port1 |= 0x02,

                Event::KeyDown {keycode: Some(Keycode::C), .. } => cpu.in_port1 |= 0x1,
                
                

                Event::KeyUp {keycode: Some(Keycode::A), .. } => cpu.in_port1 &= !0x20,
                Event::KeyUp {keycode: Some(Keycode::D), .. } => cpu.in_port1 &= !0x40,
                Event::KeyUp {keycode: Some(Keycode::W), .. } => cpu.in_port1 &= !0x10,

                Event::KeyUp {keycode: Some(Keycode::J), .. } => cpu.in_port2 &= !0x20,
                Event::KeyUp {keycode: Some(Keycode::L), .. } => cpu.in_port2 &= !0x40,
                Event::KeyUp {keycode: Some(Keycode::I), .. } => cpu.in_port2 &= !0x10,

                Event::KeyUp {keycode: Some(Keycode::Num1), .. } => cpu.in_port1 &= !0x04,
                Event::KeyUp {keycode: Some(Keycode::Num2), .. } => cpu.in_port1 &= !0x02,
                
                Event::KeyUp {keycode: Some(Keycode::C), .. } => cpu.in_port1 &= !0x1,
                _ => {}
            }
        }
        //Clearing screen before redraw.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        half_step(&mut emulator, &mut canvas, &mut cpu, true);
        half_step(&mut emulator, &mut canvas, &mut cpu, false);
        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
    
}

fn half_step(emulator: &mut Emulator, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,cpu: &mut CPU, top_half: bool) {
    let mut cycles_spent:u128 = 0;
        while cycles_spent < (CYCLES_PER_FRAME / 2) as u128 {
            let cycles = emulator.emulate(cpu);

            cycles_spent += cycles;
        }
        // println!("REDRAWING!");
        redraw_screen(canvas, cpu, top_half);
        if cpu.interrupts_enabled {
            emulator.generate_interrupt(cpu, if top_half { 1 } else { 2 });
        }

}

fn redraw_screen(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,cpu: &mut CPU, top_half: bool) {
    let width:usize = 224;
    let height:usize = 256;
    let (start_memory, start_pixel) = if top_half {
        (0x2400, 0)
    } else {
        (0x3200, 0x7000)
    };

    for offset in 0..0xE00 {
        let byte = cpu.memory[start_memory + offset];

        for bit in 0..8 {
            let color: u32 = if byte & (1 << bit) == 0 {
                0x00_00_00_00
            } else {
                0xff_ff_ff_ff
            };

            let x = (start_pixel + 8 * offset + bit) / height;
            let y = height - 1 - (start_pixel + 8 * offset + bit) % height;

            if color != 0x0 {
                draw_pixel(canvas, x as i32, y as i32);
            }
        }
    }
}

fn draw_pixel(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    let width = 224*SCALE_FACTOR;
    let height = 256*SCALE_FACTOR;

    if (y > 32) & (y <= 64) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
    } else if y > 184 && y <= 240 && x >= 0 && x <= 223 {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
    } else if ((y > 238) & (y <= 256) & (x >= 16)) && (x < 132) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
    } else {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    }


    let newx = x*width/224;
    let newy = y*height/256;
    let pixel_width = ((x + 1) * width / 224) - newx;
    let pixel_height = ((y + 1) * height / 256) - newy;
    canvas.fill_rect(Rect::new(newx, newy, pixel_width as u32, pixel_height as u32));
}