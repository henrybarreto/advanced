use std::cell::Cell;

use bevy::prelude::*;
use bevy_pixels::prelude::*;

mod emulator;

fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };

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

static EMU: emulator::Emualtor = emulator::Emualtor {};

fn main() {
    EMU.init();
    EMU.load_rom("test.gba");

    App::new()
        .add_plugins((DefaultPlugins, PixelsPlugin::default()))
        // Add systems that draw to the buffer to `Draw` schedule
        // to ensure they are rendered in the current frame.
        .add_systems(Draw, draw)
        .run();
}
