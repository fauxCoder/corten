extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::{Rect};
use sdl2::render::BlendMode;
use std::process;
use std::vec::Vec;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[macro_use]
mod events;

mod image_function;
mod pixel_art;

use pixel_art::iso;

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

fn tubby(x: u32, y: u32) -> Option<usize>
{
    let c = iso::Cuboid::new(iso::CuboidSpec { length: 100, width: 100, height: 250, ratio: 2 });

    if x >= c.texture_size.x as u32 {
        None
    } else if y >= c.texture_size.y as u32 {
        None
    }
    else {
        if iso::faces_visible(&c, &Point::new(x as i32, y as i32)) {
            Some(1)
        } else {
            Some(0)
        }
    }
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

    renderer.set_blend_mode(BlendMode::None);

    let img_func = image_function::ImageFunction::new(
        vec![
            Color::RGBA(54, 54, 54, 255),
            Color::RGBA(0, 255, 0, 255)
        ],
        tubby,
    );

    let mut v = std::vec::Vec::new();
    let result = img_func.execute(& mut v);

    let mut texture = renderer.create_texture_static(PixelFormatEnum::RGBA8888, result.size.x as u32, result.size.y as u32).unwrap();
    texture.update(None, result.data.as_slice(), (4 * result.size.x) as usize).unwrap();

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
        renderer.copy(&texture, None, Some(Rect::new(128, 128, result.size.x as u32, result.size.y as u32))).unwrap();
        renderer.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
