extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::{Rect};
use sdl2::render::BlendMode;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::process;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[macro_use]
mod events;

mod image_function;
mod pixel_art;
// mod env;

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
        } else if iso::edges_visible(&c, &Point::new(x as i32, y as i32)) {
            Some(2)
        } else {
            Some(0)
        }
    }
}

fn shorty(x: u32, y: u32) -> Option<usize>
{
    let c = iso::Cuboid::new(iso::CuboidSpec { length: 10, width: 10, height: 50, ratio: 2 });

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

    let mut canvas: Canvas<Window> = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build().unwrap();

    let mut tc: TextureCreator<WindowContext> = canvas.texture_creator();

    let img_func = image_function::ImageFunction::new(
        vec![
            Color::RGBA(255,    255,    255,      0),
            Color::RGBA( 64,    192,     64,    255),
            Color::RGBA(255,      0,      0,    255),
        ],
        tubby,
    );

    let p_img_func = image_function::ImageFunction::new(
        vec![
            Color::RGBA(  0,   0,   0,   0),
            Color::RGBA( 17,  17,  17, 255),
        ],
        shorty,
    );

    let mut v = std::vec::Vec::new();
    let result = img_func.execute(& mut v);
    let mut texture = tc.create_texture_static(PixelFormatEnum::RGBA8888, result.size.x as u32, result.size.y as u32).unwrap();
    texture.update(None, result.data.as_slice(), (4 * result.size.x) as usize).unwrap();

    let mut pv = std::vec::Vec::new();
    let presult = p_img_func.execute(& mut pv);
    let mut person = tc.create_texture_static(PixelFormatEnum::RGBA8888, presult.size.x as u32, presult.size.y as u32).unwrap();
    person.update(None, presult.data.as_slice(), (4 * presult.size.x) as usize).unwrap();

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

        canvas.set_draw_color(Color::RGBA(128, 128, 128, 255));
        canvas.clear();
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.copy(&texture, None, Some(Rect::new(128, 128, result.size.x as u32, result.size.y as u32))).unwrap();
        canvas.copy(&person, None, Some(Rect::new(164, 128, presult.size.x as u32, presult.size.y as u32))).unwrap();
        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
