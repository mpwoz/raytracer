use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
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
        // TODO: create a simple wrapper around a "File" that's just a vector of "lines" (strings).
        // Then call render() on that which joins the lines with \n, and optionally adds the ability to write to disk
        todo!();

        let newline = "\n";

        // rough estimate of capacity needed to render the whole canvas to PPM
        let mut s = String::with_capacity(self.width * self.height * 3 * 2 + 10);

        // header
        s.push_str("P3");
        s.push_str(newline);

        s.push_str(format!("{} {}\n", self.width, self.height).as_str());
        s.push_str("255\n");

        // pixel data

        s.push_str(" kdlsjflkasdjflkasjfaslk ");
        s
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
}
