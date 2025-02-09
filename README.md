# Advanced

              _                               _ 
     /\      | |                             | |
    /  \   __| |_   ____ _ _ __   ___ ___  __| |
   / /\ \ / _` \ \ / / _` | '_ \ / __/ _ \/ _` |
  / ____ \ (_| |\ V / (_| | | | | (_|  __/ (_| |
 /_/    \_\__,_| \_/ \__,_|_| |_|\___\___|\__,_|


It's an experimental and learning propose libRetro front-end to mGBA.

The primary goal of this experiment was to learn the fundamentals of Rust and C
integration through shared libraries, while also delving into the world of game
emulation.

## Results

The experiment successfully integrated the mGBA emulator with the libRetro
frontend in Rust, enabling the running of Game Boy Advance (GBA) games through
the libRetro API. The project demonstrated the capability to:

- Load and run GBA ROMs using retro_load_game.
- Capture and refresh video frames in the correct format for rendering.
- Support user input through.
- Save and load emulator states to/from disk, allowing for game state persistence.

The main challenge involved handling the different pixel formats between the
mGBA core and libRetro, which was resolved by implementing a pixel format
converter to transform RGB565 into XRGB8888. Additionally, managing safe access
to shared state data (such as button presses and video/audio frames) using
Rust's Cell and Unsafe blocks ensured a performant and functional emulator
frontend.

Overall, the project met its objectives in both understanding Rust and C
integration and creating a working libRetro frontend for the mGBA emulator.

## How to run it?

If you'd like to run it, you'll need:

- `libretro.so` in `./lib` on the repository
- `libretro.so` into some LD directory
- A GBA ROM into the current directory with the name `test.gba`
