use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_pixels::prelude::*;

mod emulator;

static EMU: emulator::Emualtor = emulator::Emualtor {};

fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else {
        return;
    };

    let format = wrapper.pixels.surface_texture_format();
    dbg!(format);

    wrapper.pixels.resize_buffer(240, 160).unwrap();
    // Get a mutable slice for the pixel buffer.
    let frame: &mut [u8] = wrapper.pixels.frame_mut();

    let (video, _) = EMU.run();

    match video {
        Some(buffer) => unsafe {
            let slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u8, buffer.len() * 4);

            frame.copy_from_slice(slice);
        },
        None => {
            println!("We don't have a buffer to display");
        }
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>) {
    let mut buttons = [0 as i16; 16];

    //   BY SELECT START   UDLR AXLR L2 R2 L3 R3
    //  [00 0      0       0000 0000 0  0  0  0]

    if keys.pressed(KeyCode::A) {
        buttons[emulator::DEVICE_ID_JOYPAD_A as usize] = 1;
    }
    if keys.pressed(KeyCode::S) {
        buttons[emulator::DEVICE_ID_JOYPAD_B as usize] = 1;
    }
    if keys.pressed(KeyCode::Return) {
        buttons[emulator::DEVICE_ID_JOYPAD_START as usize] = 1;
    }
    if keys.pressed(KeyCode::Space) {
        buttons[emulator::DEVICE_ID_JOYPAD_SELECT as usize] = 1;
    }
    if keys.pressed(KeyCode::Up) {
        buttons[emulator::DEVICE_ID_JOYPAD_UP as usize] = 1;
    }
    if keys.pressed(KeyCode::Down) {
        buttons[emulator::DEVICE_ID_JOYPAD_DOWN as usize] = 1;
    }
    if keys.pressed(KeyCode::Left) {
        buttons[emulator::DEVICE_ID_JOYPAD_LEFT as usize] = 1;
    }
    if keys.pressed(KeyCode::Right) {
        buttons[emulator::DEVICE_ID_JOYPAD_RIGHT as usize] = 1;
    }

    // vim buttons to arrow keys
    if keys.pressed(KeyCode::J) {
        buttons[emulator::DEVICE_ID_JOYPAD_DOWN as usize] = 1;
    }
    if keys.pressed(KeyCode::K) {
        buttons[emulator::DEVICE_ID_JOYPAD_UP as usize] = 1;
    }
    if keys.pressed(KeyCode::H) {
        buttons[emulator::DEVICE_ID_JOYPAD_LEFT as usize] = 1;
    }
    if keys.pressed(KeyCode::L) {
        buttons[emulator::DEVICE_ID_JOYPAD_RIGHT as usize] = 1;
    }

    EMU.push_buttons(Vec::from(buttons));
}

fn main() {
    EMU.init();
    EMU.load_rom("test.gba");

    App::new()
        .add_plugins((
            DefaultPlugins,
            PixelsPlugin {
                primary_window: Some(PixelsOptions {
                    width: 240,
                    height: 160,
                    ..Default::default()
                }),
            },
        ))
        // Add systems that draw to the buffer to `Draw` schedule
        // to ensure they are rendered in the current frame.
        .add_systems(Draw, draw)
        .add_systems(Update, keyboard_input)
        .run();
}
