extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Rect};
use std::process;
use std::vec::Vec;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[macro_use]
mod events;

mod temp;

struct_events! {
    keyboard: {
        key_left: Left,
        key_right: Right,
        key_up: Up,
        key_down: Down,
        key_escape: Escape
    },
    else: {
        quit: Quit { .. }
    }
}

fn tubby(x: u32, y: u32) -> Option<Vec<usize>>
{
    if x >= 64 {
        None
    }
    else if y >= 64 {
        None
    }
    else {
        Some(vec![0])
    }
}

fn blendy(background: Color, top: Color) -> Color
{
    top
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("Corten", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .present_vsync()
        .build().unwrap();

    let img_func = temp::ImageFunction::new(
        vec![Color::RGBA(128, 128, 128, 0)],
        tubby,
        blendy,
    );

    let mut v = std::vec::Vec::new();
    let result = img_func.execute(& mut v);

    let mut texture = renderer.create_texture_static(PixelFormatEnum::RGBA8888, result.size.x as u32, result.size.y as u32).unwrap();
    texture.update(None, result.data.as_slice(), 32).unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let mut main_loop = || {
        events.pump();

        if events.now.quit || events.now.key_escape == Some(true) {
            process::exit(1);
        }

        if events.key_left {
            // rect.x -= 4;
        }

        if events.key_right {
            // rect.x += 4;
        }

        if events.key_up {
            // rect.y -= 4;
        }

        if events.key_down {
            // rect.y += 4;
        }

        renderer.set_draw_color(Color::RGB(54, 54, 54));
        renderer.clear();
        renderer.copy(&texture, None, Some(Rect::new(100, 100, result.size.x as u32, result.size.y as u32))).unwrap();
        renderer.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
