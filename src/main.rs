extern crate termios;

use std::io::{stdout, Read, Write};
use std::io;
use std::sync::mpsc;
use std::{thread, time};

mod blocks;
mod graphic;

struct Game {
    grid: graphic::Grid,
    moving_block: Option<blocks::Block>,
    score: u32
}

impl Game {
    fn new() -> Game {
        Game { grid: graphic::Grid::new(10, 20), moving_block: None, score: 0 }
    }

    pub fn update(&mut self) -> bool {
        match self.moving_block {
            Some(_) => {}
            None => { self.moving_block = Some(blocks::Block::new(0, 5)) }
        }
        self.move_down()
    }

    pub fn rotate_right(&mut self) {
        match self.moving_block {
            Some(ref mut block) => block.rotate_right(&self.grid),
            None => {}
        }
    }

    pub fn rotate_left(&mut self) {
        match self.moving_block {
            Some(ref mut block) => block.rotate_left(&self.grid),
            None => {}
        }
    }

    pub fn move_down(&mut self) -> bool {
        match self.moving_block {
            Some(ref mut block) => {
                if !block.move_down(&self.grid) {
                    if block.over() {
                        return true;
                    }
                    block.draw(&mut self.grid);
                } else { return false; }
            }
            None => { return false; }
        }
        self.check_lines();
        self.moving_block = None;
        return false;
    }

    pub fn move_left(&mut self) {
        match self.moving_block {
            Some(ref mut block) => { block.move_left(&self.grid); }
            None => {}
        }
    }

    pub fn move_right(&mut self) {
        match self.moving_block {
            Some(ref mut block) => { block.move_right(&self.grid); }
            None => {}
        }
    }

    pub fn smack(&mut self) {
        match self.moving_block {
            Some(ref mut block) => { block.smack(&self.grid); }
            None => {}
        }
    }

    pub fn check_lines(&mut self) {
        let mut new_grid = graphic::Grid::new(self.grid.width(), self.grid.height());
        let mut index = self.grid.field.len();
        for i in (0..index).rev() {
            let mut complete = true;
            for block in self.grid.field[i].iter() {
                match *block {
                    Some(_) => {}
                    None => {
                        complete = false;
                        break;
                    }
                }
            }
            if !complete {
                index -= 1;
                new_grid.field[index] = self.grid.field[i].clone();
            } else {
                self.score += 1;
            }
        }
        self.grid = new_grid
    }

    pub fn compose(&self) -> graphic::Grid {
        let mut grid = self.grid.clone();
        match self.moving_block {
            Some(ref block) => { block.draw(&mut grid); }
            None => {}
        }
        grid
    }
}

fn main() {
    let mut console = graphic::Console::new();
    console.clear();
    console.reset();
    let (event_source, event_sink) = mpsc::channel();
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(stdin, termios::TCSANOW, &mut new_termios).unwrap();
    let mut game = Game::new();
    timer(event_source.clone());
    keypress(event_source);
    loop {
        let event = event_sink.recv().unwrap();
        match event {
            Event::Tick => {
                if game.update() {
                    stdout().write(b"\n\x1b[31mGAME OVER!\n").unwrap();
                    break;
                }
            }
            Event::KeyPress(key) => {
                match key {
                    Key::Quit => { break; }
                    Key::Left => { game.move_left(); }
                    Key::Right => { game.move_right(); }
                    Key::Down => { game.move_down(); }
                    Key::RotateLeft => { game.rotate_left(); }
                    Key::RotateRight => { game.rotate_right(); }
                    Key::Smack => { game.smack(); }
                }
            }
        }
        console.reset();
        console.draw(game.compose(), game.score);
    }
    stdout().write(b"\x1b[?25h\x1b[39m\n").unwrap();
    termios::tcsetattr(stdin, termios::TCSANOW, &termios).unwrap();
}


#[derive(Debug)]
enum Key {
    RotateLeft,
    RotateRight,
    Down,
    Left,
    Right,
    Smack,
    Quit
}

#[derive(Debug)]
enum Event {
    Tick,
    KeyPress(Key)
}

fn timer(event_source: mpsc::Sender<Event>) {
    thread::spawn(move || loop {
        event_source.send(Event::Tick).unwrap();
        thread::sleep(time::Duration::from_millis(300));
    });
}

fn keypress(event_source: mpsc::Sender<Event>) {
    thread::spawn(move || loop {
        let mut buffer = [0; 1];
        let stdin = io::stdin();
        stdin.lock().read(&mut buffer).unwrap();
        //println!("{:?}",buffer);
        event_source.send(Event::KeyPress(match buffer[0] as char {
            'w' => Key::RotateLeft,
            'e' => Key::RotateRight,
            's' => Key::Down,
            'a' => Key::Left,
            'd' => Key::Right,
            'q' => Key::Quit,
            ' ' => Key::Smack,
            _ => continue
        })).unwrap();
    });
}
