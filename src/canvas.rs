use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub(crate) fn fill(&mut self, color: Color) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.write_pixel(x, y, color);
            }
        }
    }

    pub fn new(width: usize, height: usize) -> Canvas {
        let len: usize = width * height;
        let mut pixels = Vec::with_capacity(len);

        for _ in 0..len {
            (&mut pixels).push(Color::rgb(0., 0., 0.));
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x * self.height + y
    }

    pub(crate) fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let i = self.index(x, y);
        self.pixels[i] = color
    }

    pub(crate) fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.index(x, y)]
    }

    pub fn render_as_ppm(&self) -> String {
        let newline = "\n";

        // rough estimate of capacity needed to render the whole canvas to PPM
        let mut s = String::with_capacity(self.width * self.height * 3 * 2 + 10);

        // header
        s.push_str("P3");
        s.push_str(newline);
        s.push_str(format!("{} {}{}", self.width, self.height, newline).as_str());
        s.push_str(format!("255{}", newline).as_str());

        // pixel data
        for y in 0..self.height {
            let max_line_length = 70;
            let mut line_length = 0;

            for x in 0..self.width {
                let color = self.pixel_at(x, y);
                let color_str: String = color.render_as_ppm();
                if line_length + color_str.len() > max_line_length {
                    s.push_str(newline);
                    line_length = 0;
                }
                line_length += color_str.len();
                s.push_str(color_str.as_str());
            }
            s.push_str(newline)
        }

        // return s
        s
    }

    pub fn save_to_disk(&self, location: &str) {
        let ppm = self.render_as_ppm();

        let path = Path::new(location);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(ppm.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote output to {}", display),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_initialized_black() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c.pixel_at(x, y), Color::rgb(0., 0., 0.))
            }
        }
    }

    #[test]
    fn test_canvas_pixel_setting() {
        let mut c = Canvas::new(10, 20);
        let red = Color::RED;

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn test_render_as_ppm_header() {
        let c = Canvas::new(5, 3);

        let header = c
            .render_as_ppm()
            .lines()
            .take(3)
            .fold(String::new(), |a, b| a + b + "\n");

        let expected = "P3\n\
        5 3\n\
        255\n";
        assert_eq!(header, expected);
    }

    #[test]
    fn test_render_as_ppm_pixels() {
        let mut c = Canvas::new(5, 3);
        let c1: Color = Color::rgb(1.5, 0., 0.);
        let c2: Color = Color::rgb(0., 0.5, 0.);
        let c3: Color = Color::rgb(-0.5, 0., 1.);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c
            .render_as_ppm()
            .lines()
            .skip(3)
            .fold(String::new(), |a, b| a + b + "\n");

        let expected = "\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n";
        assert_eq!(ppm, expected);
    }

    #[test]
    fn test_rendering_limits_line_length() {
        let mut canvas = Canvas::new(9, 2);
        let color = Color::rgb(1., 0.8, 0.6);
        canvas.fill(color);

        // TODO encapsulate ppm rendering/manipulation
        let ppm = canvas
            .render_as_ppm()
            .lines()
            .skip(3)
            .fold(String::new(), |a, b| a + b + "\n");

        assert_eq!(
            ppm,
            "\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
        255 204 153 255 204 153 255 204 153 255 204 153 \n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
        255 204 153 255 204 153 255 204 153 255 204 153 \n\
        "
        );
    }

    #[test]
    fn test_ppm_ends_in_newline() {
        let canvas = Canvas::new(1, 1);
        let ppm = canvas.render_as_ppm();
        let last = ppm.chars().last().unwrap();

        assert_eq!(last, '\n');
    }
}
