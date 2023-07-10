use minifb::{Key, Window, WindowOptions};
use std::cell::Cell;

mod emulator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom = &args[1];

    let emu = emulator::Emualtor {};

    emu.init();

    emu.load_rom(rom);

    let width = 240;
    let height = 160;

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~30 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(33300)));

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    /*let mut fps_timer = std::time::Instant::now();
    let mut fps_counter = 0;*/

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*fps_counter += 1;
        let elapsed = fps_timer.elapsed();
        if elapsed >= std::time::Duration::from_secs(1) {
            let fps = fps_counter as f64 / elapsed.as_secs_f64();
            window.set_title(&format!("Rust Game (FPS: {:.2})", fps));
            fps_counter = 0;
            fps_timer = std::time::Instant::now();
        }*/

        //   BY SELECT START   UDLR AXLR L2 R2 L3 R3
        //  [00 0      0       0000 0000 0  0  0  0]
        let mut buttons = vec![0; 16];

        let mini_fb_keys = window.get_keys();
        for key in mini_fb_keys {
            match key {
                Key::Enter => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_START as usize] = 1;
                }
                Key::Right => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_RIGHT as usize] = 1;
                }
                Key::Left => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_LEFT as usize] = 1;
                }
                Key::Up => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_UP as usize] = 1;
                }
                Key::Down => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_DOWN as usize] = 1;
                }
                // VIM keys.
                Key::L => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_RIGHT as usize] = 1;
                }
                Key::H => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_LEFT as usize] = 1;
                }
                Key::K => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_UP as usize] = 1;
                }
                Key::J => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_DOWN as usize] = 1;
                }
                Key::A => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_A as usize] = 1;
                }
                Key::S => {
                    buttons[emulator::RETRO_DEVICE_ID_JOYPAD_B as usize] = 1;
                }
                Key::F1 => {
                    println!("Save State");
                    emu.save_state();
                }
                Key::F2 => {
                    println!("Load State");
                    emu.load_state();
                }
                _ => {
                    println!("Unhandled Key Pressed: {:?}", key);
                }
            }
        }

        emu.push_buttons(buttons);

        let (video, _) = emu.run();

        match video {
            Some(buffer) => {
                window.update_with_buffer(&buffer, width, height).unwrap();
            }
            None => {
                println!("We don't have a buffer to display");
            }
        }
    }
}
