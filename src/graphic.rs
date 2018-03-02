extern crate rand;

use std::io::{stdout, Write};
use self::rand::Rng;

#[derive(PartialEq, Clone)]
pub enum ConsoleColor {
    Black,
    Red,
    Green,
    Orange,
    Blue,
    Purple,
    Cyan,
    LightGray
}

impl ConsoleColor {
    fn code(&self) -> &'static [u8] {
        match *self {
            ConsoleColor::Black => b"\x1b[30m",
            ConsoleColor::Red => b"\x1b[31m",
            ConsoleColor::Green => b"\x1b[32m",
            ConsoleColor::Orange => b"\x1b[33m",
            ConsoleColor::Blue => b"\x1b[34m",
            ConsoleColor::Purple => b"\x1b[35m",
            ConsoleColor::Cyan => b"\x1b[36m",
            ConsoleColor::LightGray => b"\x1b[37m"
        }
    }

    pub fn random() -> ConsoleColor {
        match rand::thread_rng().gen_range(0, 8) {
            0 => ConsoleColor::Black,
            1 => ConsoleColor::Red,
            2 => ConsoleColor::Green,
            3 => ConsoleColor::Orange,
            4 => ConsoleColor::Blue,
            5 => ConsoleColor::Purple,
            6 => ConsoleColor::Cyan,
            7 => ConsoleColor::LightGray,
            _ => ConsoleColor::Black,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Grid {
    pub field: Vec<Vec<Option<ConsoleColor>>>
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        Grid { field: vec![vec![None; width as usize]; height as usize] }
    }

    pub fn set(&mut self, x: u32, y: u32, color: ConsoleColor) {
        self.field[x as usize][y as usize] = Some(color);
    }

    pub fn get(&self, x: u32, y: u32) -> &Option<ConsoleColor> {
        &self.field[x as usize][y as usize]
    }

    pub fn check(&self, x: u32, y: u32) -> bool {
        self.field[x as usize][y as usize].is_some()
    }

    pub fn height(&self) -> u32 {
        self.field.len() as u32
    }

    pub fn width(&self) -> u32 {
        self.field[0].len() as u32
    }
}

pub struct Console {
    lastgrid: Option<Grid>,
    cursor_x: u32,
    cursor_y: u32,
    color: ConsoleColor
}

impl Console {
    pub fn clear(&mut self) {
        stdout().write(b"\x1bc\r\n").unwrap();
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    pub fn reset(&mut self) {
        stdout().write(b"\x1b[0;0f").unwrap();
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    fn move_to(&mut self, cursor_x: u32, cursor_y: u32) {
        if self.cursor_y > cursor_y {
            stdout().write(format!("\x1b[{}A;", self.cursor_y - cursor_y).as_bytes()).unwrap();
        } else if self.cursor_y < cursor_y {
            stdout().write(format!("\x1b[{}B", cursor_y - self.cursor_y).as_bytes()).unwrap();
        }

        if self.cursor_x < cursor_x {
            stdout().write(format!("\x1b[{}C", cursor_x - self.cursor_x).as_bytes()).unwrap();
        } else if self.cursor_x > cursor_x {
            stdout().write(format!("\x1b[{}D", self.cursor_x - cursor_x).as_bytes()).unwrap();
        }
        self.cursor_y = cursor_y;
        self.cursor_x = cursor_x;
    }

    pub fn new() -> Console {
        Console { lastgrid: None, cursor_x: 0, cursor_y: 0, color: ConsoleColor::Black }
    }

    pub fn draw(&mut self, grid: Grid, score: u32) {
        stdout().write(b"\x1b[107m").unwrap();
        for x in 0..grid.height() {
            for y in 0..grid.width() {
                let color = grid.get(x, y);
                match self.lastgrid {
                    Some(ref lastgrid) => if lastgrid.get(x, y) == color {
                        continue
                    },
                    None => {}
                }
                self.move_to(y as u32, x as u32);
                match *color {
                    None => {
                        stdout().write(" ".as_bytes()).unwrap();
                    }
                    Some(ref color) => {
                        if *color != self.color {
                            self.color = color.clone();
                            stdout().write(color.code()).unwrap();
                        }
                        stdout().write("█".as_bytes()).unwrap();
                    }
                }
                self.cursor_x += 1;
            }
        };
        self.move_to(0, grid.height());

        stdout().write(ConsoleColor::Black.code()).unwrap();
        self.color = ConsoleColor::Black;
        stdout().write(b"\x1b[49m").unwrap();
        stdout().write(format!("{score:>0width$}", score = score, width = grid.width() as usize).as_bytes()).unwrap();
        stdout().write(b"\x1b[?25l").unwrap();
        self.reset();
        stdout().flush().unwrap();
        self.lastgrid = Some(grid);
    }
}

