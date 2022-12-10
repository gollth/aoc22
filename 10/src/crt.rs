use std::fmt::{Display, Formatter};

use ndarray::prelude::*;

const WIDTH: usize = 39;
const HEIGHT: usize = 5;

pub struct Screen {
    pixels: Array2<Pixel>,
    width: usize,
    height: usize,
    pen: (usize, usize),

    pub sprite: i32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Pixel {
    White,
    Black,
    X,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: Array2::from_elem((WIDTH + 1, HEIGHT + 1), Pixel::X),
            width: WIDTH + 1,
            height: HEIGHT + 1,
            pen: (0, 0),
            sprite: 1,
        }
    }
}
impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "╭")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "╮")?;

        write!(f, "│ Sprite: {:2}", self.sprite)?;
        for _ in 12..=self.width {
            write!(f, " ")?;
        }
        writeln!(f, "│")?;
        write!(f, "│")?;
        for x in 0..self.width {
            write!(
                f,
                "{}",
                if self.contains_sprite(x as i32) {
                    "▅"
                } else {
                    " "
                }
            )?;
        }
        writeln!(f, "│")?;

        write!(f, "├")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "┤")?;

        for y in 0..self.height {
            write!(f, "│")?;
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.pixels[[x, y]] {
                        Pixel::X => " ",
                        Pixel::White => "█",
                        Pixel::Black => "░",
                    }
                )?;
            }
            writeln!(f, "│")?;
        }

        write!(f, "╰")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "╯")?;

        Ok(())
    }
}

impl Screen {
    fn contains_sprite(&self, x: i32) -> bool {
        let sx = self.sprite;
        vec![sx - 1, sx, sx + 1].contains(&x)
    }
    pub fn tick(&mut self) -> bool {
        // Draw the sprite with the pen
        self.pixels[self.pen] = if self.contains_sprite(self.pen.0 as i32) {
            Pixel::White
        } else {
            Pixel::Black
        };

        // Move Pen forward
        self.pen = match self.pen {
            (WIDTH, HEIGHT) => (0, 0),
            (WIDTH, y) => (0, y + 1),
            (x, y) => (x + 1, y),
        };

        if self.pen == (0, 0) {
            // Frame overflow
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increases_pen_on_every_tick() {
        let mut screen = Screen::default();
        assert_eq!(screen.pen, (0, 0));

        screen.tick();
        assert_eq!(screen.pen, (1, 0));

        for _ in 1..WIDTH {
            screen.tick();
        }

        assert_eq!(screen.pen, (WIDTH, 0));

        screen.tick();
        assert_eq!(screen.pen, (0, 1));

        for _ in 1..=HEIGHT {
            for _ in 0..=WIDTH {
                screen.tick();
            }
        }

        assert_eq!(screen.pen, (0, 0));
    }

    #[test]
    fn frame_overflow_returns_false() {
        let mut screen = Screen::default();

        for _ in 1..((screen.width) * (screen.height)) {
            assert_eq!(screen.tick(), false);
        }
        assert_eq!(screen.tick(), true);
    }

    #[test]
    fn draw_sprite_at_pen() {
        let mut screen = Screen::default();

        screen.sprite = 3;
        for _ in 1..=6 {
            screen.tick();
        }

        assert_eq!(screen.pixels[[0, 0]], Pixel::Black);
        assert_eq!(screen.pixels[[1, 0]], Pixel::Black);
        assert_eq!(screen.pixels[[2, 0]], Pixel::White);
        assert_eq!(screen.pixels[[3, 0]], Pixel::White);
        assert_eq!(screen.pixels[[4, 0]], Pixel::White);
        assert_eq!(screen.pixels[[5, 0]], Pixel::Black);
        assert_eq!(screen.pixels[[6, 0]], Pixel::X);
    }
}
