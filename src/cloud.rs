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

fn main() {
    let w: u32 = 1600;
    let h: u32 = 1600;

    let mut canvas = Canvas::new(w, h);
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in canvas.buffer.enumerate_pixels_mut() {
        *pixel = image::Rgba([255, 255, 255, 255]);
    }

    let mut x = w / 2;
    let mut y = w / 2;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in canvas.buffer.enumerate_pixels_mut() {
        *pixel = image::Rgba([200,200,200,255]);
    }

    for c in 0..10 {
        for i in 0..4000000 {
            //let mut r: u8 = (rng.gen::<f32>() * 255.0) as u8;
            //let mut g: u8 = (rng.gen::<f32>() * 255.0) as u8;
            //let mut b: u8 = (rng.gen::<f32>() * 255.0) as u8;
            let c = (rng.gen::<f32>() * 55.0) as u8;
            let color = [0,0,0, c];

            //canvas.blend(x+1, y-1, [0,0,0,50]);
            //canvas.blend(x-1, y+1, [0,0,0,50]);
            //canvas.blend(x-1, y+1, [0,0,0,50]);
            //canvas.blend(x+1, y-1, [0,0,0,50]);

            let direction = Direction::gen(rng.gen::<f32>());

            match direction {
                Direction::Up => {
                    if y == 0 {
                        continue;
                    }

                    canvas.blend(x, y - 1, color);
                    //canvas.blend(x, y - 2, color);
                    y -= 1;
                }
                Direction::Right => {
                    if x == w {
                        continue;
                    }

                    canvas.blend(x + 1, y, color);
                    //canvas.blend(x + 2, y, color);
                    x += 1;
                }
                Direction::Down => {
                    if y == h {
                        continue;
                    }

                    canvas.blend(x, y + 1, color);
                    //canvas.blend(x, y + 2, color);
                    y += 1;
                }
                Direction::Left => {
                    if x == 0 {
                        continue;
                    }

                    canvas.blend(x - 1, y, color);
                    //canvas.blend(x - 2, y, color);
                    x -= 1;
                }
            }

            //let cd = ((rng.gen::<f32>() - 0.5) * 5.0) as i32;

            //r += cd.min(255).max(0);
            //g += cd.min(255).max(0);
            //b += cd.min(255).max(0);
        }

    }

        for i in 0..1000000 {
            x = (rng.gen::<f32>() * (w as f32)) as u32;
            y = (rng.gen::<f32>() * (h as f32)) as u32;

            canvas.swap(x - 5, y - 5, x + 5, y + 5);
            canvas.swap(x - 4, y - 4, x + 6, y + 6);
            canvas.swap(x - 3, y - 3, x + 7, y + 7);
        }

        for i in 0..1000000 {
            x = (rng.gen::<f32>() * (w as f32)) as u32;
            y = (rng.gen::<f32>() * (h as f32)) as u32;

            canvas.swap(x + 5, y - 5, x - 5, y + 5);
            canvas.swap(x + 4, y - 4, x - 6, y + 6);
            canvas.swap(x + 3, y - 3, x - 7, y + 7);
        }


    // Save the image as “fractal.png”, the format is deduced from the path
    canvas.buffer.save("fractal.png").unwrap();
}
