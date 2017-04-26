extern crate sdl2;

use sdl2::rect::Point;

struct Cuboid {
    /*
                                   .UN.
                             .            .
                       .                      .UE
                UW.                       .    .
                .      .            .          .
                .          .US.                .
                .           .                  .
                .           .                  .
                .           .                  .
                .           .      (LN)        .
                .           .                  .
                .           .                 .LE
                LW.         .             .
                       .    .       .
                           .LS.

                length, LS to LE
                width,  LS to LW
                height, LS to US

                ratio is 2 for iso pixel art.
                That is, (1) up 2 across.
    */
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

fn line_right(origin: &Point, length: i32, ratio: i32, p: &Point) -> bool
{
    (p.x >= origin.x)
    &&
    (p.x < (origin.x + length))
    &&
    (((origin.y - p.y) * ratio) <= (origin.x - p.x).abs())
    &&
    (((origin.y - p.y) * ratio) >= ((origin.x - p.x).abs() - (ratio - 1)))
}

fn line_back(origin: &Point, width: i32, ratio: i32, p: &Point) -> bool
{
    (p.x <= origin.x)
    &&
    (p.x > (origin.x - width))
    &&
    (((origin.y - p.y) * ratio) <= (origin.x - p.x).abs())
    &&
    (((origin.y - p.y) * ratio) >= ((origin.x - p.x).abs() - (ratio - 1)))
}

fn line_up(origin: &Point, height: i32, p: &Point) -> bool
{
    (p.x == origin.x)
    &&
    (p.y > (origin.y - height))
    &&
    (p.y <= origin.y)
}

fn block_front(origin: &Point, length: i32, height: i32, ratio: i32, p: &Point) -> bool
{
    (p.x > origin.x)
    &&
    (p.x < (origin.x + (length - 1)))
    &&
    (((origin.y - p.y) * ratio) > (origin.x - p.x).abs())
    &&
    ((((origin.y - (height - 1)) - p.y) * ratio) < ((origin.x - p.x).abs() - (ratio - 1)))
}

fn block_side(origin: &Point, width: i32, height: i32, ratio: i32, p: &Point) -> bool
{
    (p.x < origin.x)
    &&
    (p.x > (origin.x - (width - 1)))
    &&
    (((origin.y - p.y) * ratio) > (origin.x - p.x).abs())
    &&
    ((((origin.y - (height - 1)) - p.y) * ratio) < ((origin.x - p.x).abs() - (ratio - 1)))
}

fn block_top(origin: &Point, length: i32, width: i32, ratio: i32, p: &Point) -> bool
{
    let opposite_x = (origin.x + (length - 1)) - (width - 1);
    let opposite_y = (origin.y - ((length - 1) / ratio)) - ((width - 1) / ratio);

    (((origin.y - p.y) * ratio) > (origin.x - p.x).abs())
    &&
    (((p.y - opposite_y) * ratio) > (p.x - opposite_x).abs())
}

fn corners_visible(c: &Cuboid, p: &Point) -> bool
{
    (p == &c.lower_south)
    ||
    (p == &c.lower_east)
    ||
    (p == &c.lower_west)
    ||
    (p == &c.upper_south)
    ||
    (p == &c.upper_east)
    ||
    (p == &c.upper_west)
    ||
    (p == &c.upper_north)
}

fn corners_hidden(c: &Cuboid, p: &Point) -> bool
{
    (p == &c.lower_north)
}

fn corners(c: &Cuboid, p: &Point) -> bool
{
    (corners_visible(c, p) || corners_hidden(c, p))
}

fn edges_visible(c: &Cuboid, p: &Point) -> bool
{
    line_right(&c.lower_south, c.length, c.ratio, p)
    ||
    line_right(&c.upper_south, c.length, c.ratio, p)
    ||
    line_right(&c.upper_west, c.length, c.ratio, p)
    ||
    line_back(&c.lower_south, c.width, c.ratio, p)
    ||
    line_back(&c.upper_east, c.width, c.ratio, p)
    ||
    line_back(&c.upper_south, c.width, c.ratio, p)
    ||
    line_up(&c.lower_south, c.height, p)
    ||
    line_up(&c.lower_east, c.height, p)
    ||
    line_up(&c.lower_west, c.height, p)
}

fn edges_hidden(c: &Cuboid, p: &Point) -> bool
{
    line_right(&c.lower_west, c.length, c.ratio, p)
    ||
    line_back(&c.lower_east, c.width, c.ratio, p)
    ||
    line_up(&c.lower_north, c.height, p)
}

fn edges(c: &Cuboid, p: &Point) -> bool
{
    (edges_visible(c, p) || edges_hidden(c, p))
}

fn face_front(c: &Cuboid, p: &Point) -> bool
{
    block_front(&c.lower_south, c.length, c.height, c.ratio, p)
}

fn face_side(c: &Cuboid, p: &Point) -> bool
{
    block_side(&c.lower_south, c.width, c.height, c.ratio, p)
}

fn face_top(c: &Cuboid, p: &Point) -> bool
{
    block_top(&c.upper_south, c.length, c.width, c.ratio, p)
}

fn face_back(c: &Cuboid, p: &Point) -> bool
{
    block_front(&c.lower_west, c.length, c.height, c.ratio, p)
}

fn face_far_side(c: &Cuboid, p: &Point) -> bool
{
    block_side(&c.lower_east, c.width, c.height, c.ratio, p)
}

fn face_bottom(c: &Cuboid, p: &Point) -> bool
{
    block_top(&c.lower_south, c.length, c.width, c.ratio, p)
}

fn faces_visible(c: &Cuboid, p: &Point) -> bool
{
    face_front(c, p)
    ||
    face_side(c, p)
    ||
    face_top(c, p)
}

fn faces_hidden(c: &Cuboid, p: &Point) -> bool
{
    face_back(c, p)
    ||
    face_far_side(c, p)
    ||
    face_bottom(c, p)
}

fn faces(c: &Cuboid, p: &Point) -> bool
{
    (faces_visible(c, p) || faces_hidden(c, p))
}
