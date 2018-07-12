# RustyGUI
GUI toolkit based upon SDL2.

## System Dependencies
* Rust and Cargo
* SDL2 ([installation described here](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md#sdl20-development-libraries))
* SDL2 image ([Download here](https://www.libsdl.org/projects/SDL_image/), same install process as SDL2)
* SDL2 mixer ([Download here](https://www.libsdl.org/projects/SDL_mixer/), same install process as SDL2)
* SDL2 ttf ([Download here](https://www.libsdl.org/projects/SDL_ttf/), same install process as SDL2)
* SDL2 gfx ([Download here](https://sourceforge.net/projects/sdl2gfx/), same install process as SDL2)

On macOS, the easiest way to install all SDL2 dependencies is to use [Homebrew](https://github.com/Homebrew/brew):

`brew install sdl2 sdl2_image sdl2_mixer sdl2_ttf sdl2_gfx`

## Building
`cargo build`

## Running
`cargo run`
