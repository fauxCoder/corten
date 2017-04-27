extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::option::Option;
use std::vec::Vec;

pub struct ImageFunction {
    palette: Vec<Color>,
    function: fn(u32, u32) -> Option<usize>,
}

pub struct ImageResult<'a> {
    pub size: Point,
    pub data: &'a Vec<u8>,
}

impl ImageFunction {
    pub fn new(palette: Vec<Color>, function: fn(u32, u32) -> Option<usize>) -> ImageFunction {
        ImageFunction {
            palette: palette,
            function: function,
        }
    }

    pub fn execute<'a>(&self, v: &'a mut Vec<u8>) -> ImageResult<'a>
    {
        let mut size_x = 0;
        let mut size_y = 0;

        let mut y = 0;
        let mut valid_y = true;
        while valid_y {
            let mut x = 0;
            let mut valid_x = true;
            while valid_x {
                match (self.function)(x, y) {
                    Some(index) => {
                        size_x = if x >= size_x { x + 1 } else { size_x };
                        size_y = if y >= size_y { y + 1 } else { size_y };

                        let c = self.palette[index];

                        let (r, g, b, a) = c.rgba();
                        v.push(a);
                        v.push(b);
                        v.push(g);
                        v.push(r);
                    },
                    None => {
                        valid_x = false;

                        if x == 0 {
                            valid_y = false;
                        }
                    },
                }
                x += 1;
            }
            y += 1;
        }

        ImageResult {
            size: Point::new(size_x as i32, size_y as i32),
            data: v
        }
    }
}
