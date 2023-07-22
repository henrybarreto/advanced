use std::cell::Cell;
use std::io::{Read, Write};
use std::ptr;

mod libretro;

static mut BUTTONS_PRESSED: Cell<Option<Vec<i16>>> = Cell::new(None);

static mut VIDEO_FRAME_DATA: Cell<Option<Vec<u32>>> = Cell::new(None);
static mut AUDIO_FRAME_DATA: Cell<Option<Vec<i16>>> = Cell::new(None);

/*pub const RETRO_DEVICE_ID_JOYPAD_B: i16 = 0;
pub const RETRO_DEVICE_ID_JOYPAD_Y: i16 = 1;
pub const RETRO_DEVICE_ID_JOYPAD_SELECT: i16 = 2;
pub const RETRO_DEVICE_ID_JOYPAD_START: i16 = 3;
pub const RETRO_DEVICE_ID_JOYPAD_UP: i16 = 4;
pub const RETRO_DEVICE_ID_JOYPAD_DOWN: i16 = 5;
pub const RETRO_DEVICE_ID_JOYPAD_LEFT: i16 = 6;
pub const RETRO_DEVICE_ID_JOYPAD_RIGHT: i16 = 7;
pub const RETRO_DEVICE_ID_JOYPAD_A: i16 = 8;
pub const RETRO_DEVICE_ID_JOYPAD_X: i16 = 9;
pub const RETRO_DEVICE_ID_JOYPAD_L: i16 = 10;
pub const RETRO_DEVICE_ID_JOYPAD_R: i16 = 11;
pub const RETRO_DEVICE_ID_JOYPAD_L2: i16 = 12;
pub const RETRO_DEVICE_ID_JOYPAD_R2: i16 = 13;
pub const RETRO_DEVICE_ID_JOYPAD_L3: i16 = 14;
pub const RETRO_DEVICE_ID_JOYPAD_R3: i16 = 15;*/

pub const DEVICE_ID_JOYPAD_B: i16 = 0;
pub const DEVICE_ID_JOYPAD_Y: i16 = 1;
pub const DEVICE_ID_JOYPAD_SELECT: i16 = 2;
pub const DEVICE_ID_JOYPAD_START: i16 = 3;
pub const DEVICE_ID_JOYPAD_UP: i16 = 4;
pub const DEVICE_ID_JOYPAD_DOWN: i16 = 5;
pub const DEVICE_ID_JOYPAD_LEFT: i16 = 6;
pub const DEVICE_ID_JOYPAD_RIGHT: i16 = 7;
pub const DEVICE_ID_JOYPAD_A: i16 = 8;
pub const DEVICE_ID_JOYPAD_X: i16 = 9;

//   BY SELECT START   UDLR AXLR L2 R2 L3 R3
//  [00 0      0       0000 0000 0  0  0  0]

unsafe extern "C" fn my_environment(
    _cmd: std::os::raw::c_uint,
    _data: *mut std::os::raw::c_void,
) -> bool {
    // Function body goes here
    // Perform operations using the cmd and data arguments

    /*match cmd {
        libretro::RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            /*let pixel_format = *(data as *const u32);
            let pixel_format_as_enum = pixel_format;
            match pixel_format_as_enum {
                libretro::retro_pixel_format_RETRO_PIXEL_FORMAT_0RGB1555 => {
                    panic!("Core will send us pixel data in the RETRO_PIXEL_FORMAT_0RGB1555 format")
                }
                libretro::retro_pixel_format_RETRO_PIXEL_FORMAT_RGB565 => {
                    panic!("Core will send us pixel data in the RETRO_PIXEL_FORMAT_RGB565 format") <-----
                }
                libretro::retro_pixel_format_RETRO_PIXEL_FORMAT_XRGB8888 => {
                    panic!("Core will send us pixel data in the RETRO_PIXEL_FORMAT_XRGB8888 format")
                }
                _ => panic!("Core is trying to use an Unknown Pixel Format"),
            }*/

            return true;
        }
        _ => println!("libretro_environment_callback Called with command: {}", cmd),
    }*/

    return false;
}

/// rgb565 is the default pixel format of mGBA core, however, the minifb is not able to display it. So we need to
/// convert it to xrgb8888
///
/// ### RETRO_PIXEL_FORMAT_RGB565
/// You can read this as Red,Green,Blue
///
/// - 5 bits for red
/// - 6 bits for green (Humans are better at seeing moire shades of green than red/blue)
/// - 5 bits for blue
/// - 16 bits total (2 bytes per pixel))
///
///
/// ### RETRO_PIXEL_FORMAT_XRGB8888
/// You can read this as Nothing,Red,Green,Blue
///
/// - 8 bits at the start that are unused (X)
/// - 8 bits for red
/// - 8 bits for green
/// - 8 bits for blue
/// - 32 bits total (4 bytes per pixel)
fn convert_pixel_array_from_rgb565_to_xrgb8888(
    frame: &[u8],
    width: usize,
    height: usize,
    pitch: usize,
) -> Box<[u32]> {
    // TODO: undestand EACH LINE of this function!

    let frame_size = width * height;
    let mut converted_frame = vec![0u32; frame_size].into_boxed_slice();

    for y in 0..height {
        let row_offset = y * pitch;
        let row_bytes = &frame[row_offset..row_offset + (width * 2)];

        for x in 0..width {
            let pixel_offset = x * 2;

            let byte1 = row_bytes[pixel_offset] as u32;
            let byte2 = row_bytes[pixel_offset + 1] as u32;

            // One pixel are two bytes or 16 bits. (byte1 and byte2).
            //                           byte2    byte1
            // pixel = 00000000 00000000 ^^^^^^^^ ^^^^^^^^
            let pixel = (byte2 << 8) | byte1;

            // Extracting color components from RGB565 format. (RRRRR GGGGGG BBBBB)
            //
            // RGB565 format:
            // RED   GREEN  BLUE
            // 00000 000000 00000 <- 16 bits (2 bytes)
            //
            // Pixel example:
            // 00000000 00000000 11111000 00011111
            //
            //                             RED   GREEN  BLUE
            // let r = ((00000000 00000000 11111 000000 11111 >> 11 = 00000000 00000000 11111 &
            //                                                        00000000 00000000 11111 =
            //                                                        00000000 00000000 11111) << 3 = 00000000 00000000 00000000 11111000)
            let r = ((pixel >> 11) & 0b00011111) << 3;
            //                             RED   GREEN  BLUE
            // let g = ((00000000 00000000 11111 000000 11111 >> 5 = 00000000 00000000 11111000 000 &
            //                                                       00000000 00000000 00000111 111 =
            //                                                       00000000 00000000 00000000 000 = 00000000 00000000 00000000 00000000
            let g = ((pixel >> 5) & 0b00111111) << 2;
            //                             RED   GREEN  BLUE
            // let b = ((00000000 00000000 11111 000000 11111 &
            //           00000000 00000000 11111 000000 11111 =
            //           00000000 00000000 11111 000000 11111) << 3 = 00000000 00000000 00000000 11111000)
            let b = (pixel & 0x1F) << 3;

            // Combining color components into XRGB8888 format. (AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB)
            //
            //                     ALPHA        RED         GREEN      BLUE
            //                     11111111     00000000    00000000   00000000  <- 32 bits (4 bytes)
            //                     ^^^^^^^^     xxxxxxxx    00000000   00000000  <- 24 bits (3 bytes)
            //                     ^^^^^^^^     ^^^^^^^^    xxxxxxxx   00000000  <- 16 bits (2 bytes)
            //                     ^^^^^^^^     ^^^^^^^^    ^^^^^^^^   xxxxxxxx  <- 8 bits  (1 byte)
            //
            // Pixel example:
            //  (0xFF << 24) =    11111111 00000000 00000000 00000000
            //  (r << 16) =       00000000 11111000 00000000 00000000
            //  (g << 8) =        00000000 00000000 00000000 00000000
            //  (b) =             00000000 00000000 00000000 11111000
            //                    -----------------------------------
            //                    11111111 11111000 00000000 11111000
            let xrgb8888_pixel = (0xFF << 24) | (r << 16) | (g << 8) | b;

            // Storing the converted pixel in the output frame
            converted_frame[y * width + x] = xrgb8888_pixel;
        }
    }

    converted_frame
}

unsafe extern "C" fn my_video_refresh(
    data: *const std::os::raw::c_void,
    width: std::os::raw::c_uint,
    height: std::os::raw::c_uint,
    pitch: usize,
) {
    if data == ptr::null() {
        println!("frame_buffer_data was null");
        return;
    }

    // let length_of_frame_buffer = width * height;
    let length_of_frame_buffer = ((pitch as u32) * height) * 2 as u32;

    //let buffer_slice = std::slice::from_raw_parts(data as *const u8, pitch * height as usize);
    let buffer_slice =
        std::slice::from_raw_parts(data as *const u8, length_of_frame_buffer as usize);

    let result = convert_pixel_array_from_rgb565_to_xrgb8888(
        buffer_slice,
        width as usize,
        height as usize,
        pitch,
    );

    let buffer_vec = Vec::from(result);

    //CURRENT_EMULATOR_STATE.frame_buffer = Some(buffer_vec);
    VIDEO_FRAME_DATA.set(Some(buffer_vec));
}

unsafe extern "C" fn my_audio_sample_batch(_data: *const i16, _frames: usize) -> usize {
    return 0;
}

unsafe extern "C" fn my_input_poll() {
    // println!("input poll");
}

unsafe extern "C" fn my_input_state(
    _port: std::os::raw::c_uint,
    _device: std::os::raw::c_uint,
    _index: std::os::raw::c_uint,
    id: std::os::raw::c_uint,
) -> i16 {
    let is_pressed = match BUTTONS_PRESSED.get_mut() {
        Some(buttons_pressed) => buttons_pressed[id as usize],
        None => 0,
    };

    return is_pressed;
}

unsafe extern "C" fn my_audio_sample(_left: i16, _right: i16) {}

pub struct Emualtor {}

impl Emualtor {
    pub fn init(&self) {
        unsafe {
            libretro::retro_set_environment(Some(my_environment));

            libretro::retro_set_video_refresh(Some(my_video_refresh));

            libretro::retro_set_audio_sample(Some(my_audio_sample));

            libretro::retro_set_audio_sample_batch(Some(my_audio_sample_batch));

            libretro::retro_set_input_poll(Some(my_input_poll));

            libretro::retro_set_input_state(Some(my_input_state));

            libretro::retro_set_input_poll(Some(my_input_poll));

            libretro::retro_init();
        }
    }

    pub fn load_rom(&self, rom: &str) {
        let mut file = std::fs::File::open(rom).expect("could not open the rom's file");
        let size = file.metadata().unwrap().len() as usize;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)
            .expect("could not read the rom's file");

        let info = libretro::retro_game_info {
            path: rom.as_ptr() as *const libc::c_char,
            data: buffer.as_ptr() as *const libc::c_void,
            size,
            meta: std::ptr::null(),
        };

        unsafe {
            if !libretro::retro_load_game(&info) {
                return;
            }
        }
    }

    pub fn save_state(&self) {
        unsafe {
            let size = libretro::retro_serialize_size();
            let mut buffer: Vec<u8> = vec![0; size];

            if !libretro::retro_serialize(buffer.as_mut_ptr() as *mut std::ffi::c_void, size) {
                return;
            }

            let mut file =
                std::fs::File::create("save_state").expect("could not create the save state");
            file.write_all(&buffer).unwrap();
        }
    }

    pub fn load_state(&self) {
        let mut file =
            std::fs::File::open("save_state").expect("could not open the save state file");
        let size = file.metadata().unwrap().len() as usize;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer).unwrap();

        unsafe {
            if !libretro::retro_unserialize(buffer.as_mut_ptr() as *mut std::ffi::c_void, size) {
                return;
            }
        }
    }

    pub fn push_buttons(&self, buttons: Vec<i16>) {
        unsafe {
            BUTTONS_PRESSED.set(Some(buttons));
        }
    }

    pub fn run(&self) -> (Option<Vec<u32>>, Option<(i16, i16)>) {
        let video;

        unsafe {
            libretro::retro_run();
            video = VIDEO_FRAME_DATA.replace(None);
        }

        return (video, None);
    }
}
