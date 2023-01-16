//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;
use image::Pixel;
use rand::Rng;

struct Canvas {
    width: u32,
    height: u32,
    buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Canvas {
    fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            buffer: image::ImageBuffer::new(width, height),
        }
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn draw(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if !self.is_inside(x, y) {
            return;
        }

        let pixel = self.buffer.get_pixel_mut(x, y);
        *pixel = image::Rgba(color);
    }

    fn blend(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if !self.is_inside(x, y) {
            return;
        }

        let existing = self.buffer.get_pixel_mut(x, y);
        existing.blend(&image::Rgba(color));
    }

    fn swap(&mut self, x: u32, y: u32, x2: u32, y2: u32) {
        if !self.is_inside(x, y) || !self.is_inside(x2, y2) {
            return;
        }

        let a = self.buffer.get_pixel_mut(x, y);
        let ac = *a as image::Rgba<u8>;
        drop(a);

        let b = self.buffer.get_pixel_mut(x2, y2);
        let bc = *b as image::Rgba<u8>;
        drop(b);

        let a = self.buffer.get_pixel_mut(x, y);
        *a = bc;
        drop(a);

        let b = self.buffer.get_pixel_mut(x2, y2);
        *b = ac;
        drop(b);
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn gen(r: f32) -> Self {
        match r {
            0.0..=0.25 => Self::Up,
            0.25..=0.5 => Self::Right,
            0.5..=0.75 => Self::Down,
            0.75..=1.0 => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Posn {
    x: usize,
    y: usize,
}

impl Posn {
    fn shift(&self, xd: isize, yd: isize) -> Posn {
        Posn {
            x: ((self.x as isize) + xd) as usize,
            y: ((self.y as isize) + yd) as usize,
        }
    }

    fn neighbor(&self, dir: Direction) -> Posn {
        match dir {
            Direction::Up => Posn {
                x: self.x,
                y: self.y - 2,
            },
            Direction::Down => Posn {
                x: self.x,
                y: self.y + 2,
            },
            Direction::Left => Posn {
                x: self.x - 2,
                y: self.y,
            },
            Direction::Right => Posn {
                x: self.x + 2,
                y: self.y,
            },
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    bits: Vec<bool>,
    trail: Vec<Posn>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        let start = Posn {
            x: width / 2 + 1,
            y: height / 2 + 1,
        };

        let mut map = Map {
            width,
            height,
            bits: vec![false; width * height],
            trail: vec![],
        };

        map.visit(start);

        map.trail.push(start);

        map
    }

    fn check(&self, posn: Posn) -> bool {
        let i = (posn.y * self.width) + posn.x;
        if i >= self.bits.len() {
            false
        } else {
            self.bits[i]
        }
    }

    fn visit(&mut self, posn: Posn) {
        let i = (posn.y * self.width) + posn.x;
        if i < self.bits.len() {
            self.bits[i] = true;
        }
    }

    fn posn(&self) -> Posn {
        *self.trail.last().unwrap()
    }

    fn within_bounds(&self, posn: Posn) -> bool {
        posn.x >= 0 && posn.y >= 0 && posn.x < self.width && posn.y < self.height
    }

    fn can_move(&self, dir: Direction) -> bool {
        let neighbor = self.posn().neighbor(dir);
        self.within_bounds(neighbor) && !self.check(neighbor)
    }

    fn is_stuck(&self) -> bool {
        !self.can_move(Direction::Left)
            && !self.can_move(Direction::Right)
            && !self.can_move(Direction::Up)
            && !self.can_move(Direction::Down)
    }

    fn move_to(&mut self, dir: Direction) {
        let posn = self.posn();
        let new_posn = self.posn().neighbor(dir);
        match dir {
            Direction::Left => {
                self.visit(posn.shift(-1, 0));
                self.visit(posn.shift(-2, 0));
            }
            Direction::Right => {
                self.visit(posn.shift(1, 0));
                self.visit(posn.shift(2, 0));
            }
            Direction::Up => {
                self.visit(posn.shift(0, -1));
                self.visit(posn.shift(0, -2));
            }
            Direction::Down => {
                self.visit(posn.shift(0, 1));
                self.visit(posn.shift(0, 2));
            }
        }
        self.trail.push(new_posn);
    }

    fn iter<'a>(&'a self) -> MapIterator<'a> {
        MapIterator { map: self, i: 0 }
    }
}

struct MapIterator<'a> {
    map: &'a Map,
    i: usize,
}

impl Iterator for MapIterator<'_> {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.map.width * self.map.height {
            None
        } else {
            let y = (self.i - (self.i % self.map.width)) / self.map.width;
            let x = self.i % self.map.width;
            let visited = self.map.check(Posn { x, y });
            self.i += 1;
            Some((x, y, visited))
        }
    }
}

fn main() {
    let w: u32 = 901;
    let h: u32 = 901;

    let mut canvas = Canvas::new(w, h);
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in canvas.buffer.enumerate_pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in canvas.buffer.enumerate_pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }

    let mut map = Map::new(w as usize, h as usize);

    let mut x = w as usize / 2;
    let mut y = w as usize / 2;

    'mainLoop: while true {
        let dir = Direction::gen(rng.gen());

        while map.is_stuck() {
            map.trail.pop();
            if map.trail.len() == 0 {
                break 'mainLoop;
            }
        }

        if map.can_move(dir) {
            map.move_to(dir);
        }
    }

    for (x, y, visited) in map.iter() {
        if visited {
            canvas.draw(x as u32, y as u32, [0, 0, 0, 255]);
        }
    }

    canvas.buffer.save("map.png").unwrap();
}
