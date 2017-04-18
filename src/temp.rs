extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::option::Option;
use std::vec::Vec;

pub struct ImageFunction {
    palette: Vec<Color>,
    function: fn(u32, u32) -> Option<Vec<usize>>,
    blending: fn(Color, Color) -> Color,
}

pub struct ImageResult<'a> {
    pub size: Point,
    pub data: &'a Vec<u8>,
}

impl ImageFunction {
    pub fn new(palette: Vec<Color>, function: fn(u32, u32) -> Option<Vec<usize>>, blending: fn(Color, Color) -> Color) -> ImageFunction {
        ImageFunction {
            palette: palette,
            function: function,
            blending: blending,
        }
    }

    pub fn execute<'a>(&self, v: &'a mut Vec<u8>) -> ImageResult<'a>
    {
        let mut size_x = 0;
        let mut size_y = 0;

        let mut y = 0;
        let mut validY = true;
        while validY {
            let mut x = 0;
            let mut validX = true;
            while validX {
                let result = (self.function)(x, y);

                if result == None {
                    validX = false;

                    if x == 0 {
                        validY = false;
                    }
                }
                else
                {
                    let rv = result.unwrap();

                    size_x = if x >= size_x { x + 1 } else { size_x };
                    size_y = if y >= size_y { y + 1 } else { size_y };

                    let mut blended = Color::RGBA(255, 255, 255, 0);

                    for i in rv {
                        let c = self.palette[i];

                        // blended = (self.blending)(blended, c);
                        blended = c;
                    }

                    let (r, g, b, a) = blended.rgba();
                    v.push(r);
                    v.push(g);
                    v.push(b);
                    v.push(a);
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

struct Cuboid {
    length: i32,
    width: i32,
    height: i32,
    ratio: i32,

    texture_size: Point,

    lower_south: Point,
    lower_east: Point,
    lower_west: Point,
    lower_north: Point,
    upper_south: Point,
    upper_east: Point,
    upper_west: Point,
    upper_north: Point,
}

struct CuboidSpec {
    length: i32,
    width: i32,
    height: i32,
    ratio: i32,
}

impl Cuboid {
    fn new(s: CuboidSpec) -> Cuboid {
        let texture_size = Point::new(
            s.width + s.length - 1,
            (s.height - 1) + (s.length / s.ratio) + (s.width / s.ratio) - 1,
        );
        let lower_south = Point::new(
            s.width - 1,
            texture_size.y - 1,
        );
        let lower_east = Point::new(
            lower_south.x + (s.length - 1),
            lower_south.y - ((s.length - 1) / s.ratio),
        );
        let lower_west = Point::new(
            lower_south.x - (s.width - 1),
            lower_south.y - ((s.width - 1) / s.ratio),
        );
        let lower_north = Point::new(
            lower_east.x - (s.width - 1),
            lower_east.y - ((s.width - 1) / s.ratio),
        );
        let upper_south = Point::new(
            lower_south.x,
            lower_south.y - (s.height - 1),
        );
        let upper_east = Point::new(
            lower_east.x,
            upper_south.y - ((s.length - 1) / s.ratio),
        );
        let upper_west = Point::new(
            lower_west.x,
            upper_south.y- ((s.width - 1) / s.ratio),
        );
        let upper_north = Point::new(
            lower_north.x,
            upper_east.y - ((s.width - 1) / s.ratio),
        );

        Cuboid {
            length: s.length,
            width: s.width,
            height: s.height,
            ratio: s.ratio,

            texture_size: texture_size,
            lower_south: lower_south,
            lower_east: lower_east,
            lower_west: lower_west,
            lower_north: lower_north,
            upper_south: upper_south,
            upper_east: upper_east,
            upper_west: upper_west,
            upper_north: upper_north,
        }
    }
}

fn line_right(origin: Point, length: i32, ratio: i32, p: Point) -> bool
{
    (p.x >= origin.x)
    &&
    (p.x < (origin.x + length))
    &&
    (((origin.y - p.y) * ratio) <= (origin.x - p.x).abs())
    &&
    (((origin.y - p.y) * ratio) >= ((origin.x - p.x).abs() - (ratio - 1)))
}

fn line_back(origin: Point, width: i32, ratio: i32, p: Point) -> bool
{
    (p.x <= origin.x)
    &&
    (p.x > (origin.x - width))
    &&
    (((origin.y - p.y) * ratio) <= (origin.x - p.x).abs())
    &&
    (((origin.y - p.y) * ratio) >= ((origin.x - p.x).abs() - (ratio - 1)))
}

fn line_up(origin: Point, height: i32, p: Point) -> bool
{
    (p.x == origin.x)
    &&
    (p.y > (origin.y - height))
    &&
    (p.y <= origin.y)
}

fn block_front(origin: Point, length: i32, height: i32, ratio: i32, p: Point) -> bool
{
    (p.x > origin.x)
    &&
    (p.x < (origin.x + (length - 1)))
    &&
    (((origin.y - p.y) * ratio) > (origin.x - p.x).abs())
    &&
    ((((origin.y - (height - 1)) - p.y) * ratio) < ((origin.x - p.x).abs() - (ratio - 1)))
}

fn block_side(origin: Point, width: i32, height: i32, ratio: i32, p: Point) -> bool
{
    (p.x < origin.x)
    &&
    (p.x > (origin.x - (width - 1)))
    &&
    (((origin.y - p.y) * ratio) > (origin.x - p.x).abs())
    &&
    ((((origin.y - (height - 1)) - p.y) * ratio) < ((origin.x - p.x).abs() - (ratio - 1)))
}

fn block_top(origin: Point, length: i32, width: i32, ratio: i32, p: Point) -> bool
{
    let opposite_x = (origin.x + (length - 1)) - (width - 1);
    let opposite_y = (origin.y - ((length - 1) / ratio)) - ((width - 1) / ratio);

    (((origin.y - p.y) * ratio) > (origin.x - p.x).abs())
    &&
    (((p.y - opposite_y) * ratio) > (p.x - opposite_x).abs())
}
