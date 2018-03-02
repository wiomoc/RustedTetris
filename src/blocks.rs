extern crate rand;

use self::rand::Rng;
use graphic;

#[derive(Clone)]
pub enum Rotation {
    D0,
    D90,
    D180,
    D270
}

impl Rotation {
    pub fn random() -> Rotation {
        match rand::thread_rng().gen_range(0, 4) {
            0 => Rotation::D0,
            1 => Rotation::D90,
            2 => Rotation::D180,
            3 => Rotation::D270,
            _ => Rotation::D0,
        }
    }
}

struct Rectangle {
    x: i8,
    y: i8
}

pub struct Block {
    block_type: BlockType,
    x: u32,
    y: u32,
    rotation: Rotation,
    color: graphic::ConsoleColor
}

impl Block {
    pub fn new(x: u32, y: u32) -> Block {
        Block { block_type: BlockType::random(), rotation: Rotation::random(), color: graphic::ConsoleColor::random(), x: x, y: y }
    }

    pub fn draw(&self, grid: &mut graphic::Grid) {
        self.block_type.draw(self.x, self.y, self.rotation.clone(), self.color.clone(), grid)
    }

    pub fn rotate_right(&mut self, grid: &graphic::Grid) {
        let rotation = match self.rotation {
            Rotation::D0 => Rotation::D90,
            Rotation::D90 => Rotation::D180,
            Rotation::D180 => Rotation::D270,
            Rotation::D270 => Rotation::D0
        };
        if !(self.block_type.bounds(self.y as i32, rotation.clone(), grid) || self.block_type.collison(self.x, self.y, rotation.clone(), grid)) {
            self.rotation = rotation
        }
    }

    pub fn rotate_left(&mut self, grid: &graphic::Grid) {
        let rotation = match self.rotation {
            Rotation::D0 => Rotation::D270,
            Rotation::D90 => Rotation::D0,
            Rotation::D180 => Rotation::D90,
            Rotation::D270 => Rotation::D180
        };
        if !(self.block_type.bounds(self.y as i32, rotation.clone(), grid) || self.block_type.collison(self.x, self.y, rotation.clone(), grid)) {
            self.rotation = rotation
        }
    }

    pub fn move_down(&mut self, grid: &graphic::Grid) -> bool {
        self.x += 1;
        if self.block_type.collison(self.x, self.y, self.rotation.clone(), grid) {
            self.x -= 1;
            return false;
        }
        true
    }

    pub fn move_left(&mut self, grid: &graphic::Grid) -> bool {
        if self.block_type.bounds((self.y as i32) - 1, self.rotation.clone(), grid) || self.block_type.collison(self.x, ((self.y as i32) - 1) as u32, self.rotation.clone(), grid) {
            return false;
        } else {
            self.y -= 1;
            return true;
        }
    }

    pub fn move_right(&mut self, grid: &graphic::Grid) -> bool {
        if self.block_type.bounds((self.y as i32) + 1, self.rotation.clone(), grid) || self.block_type.collison(self.x, ((self.y as i32) + 1) as u32, self.rotation.clone(), grid) {
            return false;
        } else {
            self.y += 1;
            return true;
        }
    }

    pub fn smack(&mut self, grid: &graphic::Grid) {
        while !self.block_type.collison(self.x, self.y, self.rotation.clone(), grid) {
            self.x += 1;
        }
        self.x -= 1;
    }

    pub fn over(&self) -> bool {
        self.block_type.over(self.x, self.rotation.clone())
    }
}

pub enum BlockType {
    T,
    I,
    O,
    J,
    L,
    S,
    Z
}
macro_rules! carect {
    ( $ ( $ es: expr), * $ (, ) * ) => {
        {
          const C: & 'static [Rectangle] = &[ $ ( $ es), * ];
            C
        }
    };
}

impl BlockType {
    fn rectangles(&self, rotation: Rotation) -> &'static [Rectangle] {
        match *self {
            BlockType::T => {
                match rotation {
                    Rotation::D0 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 0, y: 1 }),
                    Rotation::D90 => carect!(Rectangle { x: 0, y: - 1 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 1, y: 0 }),
                    Rotation::D180 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 0, y: - 1 }),
                    Rotation::D270 => carect!(Rectangle { x: 0, y: - 1 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: - 1, y: 0 })
                }
            }
            BlockType::I => {
                match rotation {
                    Rotation::D0 | Rotation::D180 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 2, y: 0 }),
                    Rotation::D90 | Rotation::D270 => carect!(Rectangle { x: 0, y: - 1 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 0, y: 2 })
                }
            }
            BlockType::O => {
                carect!(Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 1, y: 1 })
            }
            BlockType::J => {
                match rotation {
                    Rotation::D0 => carect!(Rectangle { x: - 1, y: 1 }, Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }),
                    Rotation::D90 => carect!(Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 0, y: - 1 }, Rectangle { x: 0, y: - 2 }),
                    Rotation::D180 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 1, y: - 1 }),
                    Rotation::D270 => carect!(Rectangle { x: - 1, y: - 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 0, y: 2 }),
                }
            }

            BlockType::L => {
                match rotation {
                    Rotation::D0 => carect!(Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 0, y: 2 }),
                    Rotation::D90 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: - 1, y: - 1 }, Rectangle { x: 1, y: - 0 }),
                    Rotation::D180 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: - 1 }, Rectangle { x: 0, y: - 2 }),
                    Rotation::D270 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 1, y: 1 }),
                }
            }
            BlockType::S => {
                match rotation {
                    Rotation::D0 | Rotation::D180 => carect!(Rectangle { x: - 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 1, y: 1 }),
                    Rotation::D90 | Rotation::D270 => carect!(Rectangle { x: 0, y: 1 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 1, y: - 1 })
                }
            }
            BlockType::Z => {
                match rotation {
                    Rotation::D0 | Rotation::D180 => carect!(Rectangle { x: -1, y: 1 }, Rectangle { x: 0, y: 1 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 1, y: 0 }),
                    Rotation::D90 | Rotation::D270 => carect!(Rectangle { x: 1, y: 1 }, Rectangle { x: 1, y: 0 }, Rectangle { x: 0, y: 0 }, Rectangle { x: 0, y: -1 })
                }
            }
        }
    }

    pub fn draw(&self, x: u32, y: u32, rotation: Rotation, color: graphic::ConsoleColor, grid: &mut graphic::Grid) {
        for rect in self.rectangles(rotation) {
            grid.set((((x as i32) + (rect.x as i32)) as u32), (((y as i32) + (rect.y as i32)) as u32), color.clone());
        }
    }

    pub fn collison(&self, x: u32, y: u32, rotation: Rotation, grid: &graphic::Grid) -> bool {
        for rect in self.rectangles(rotation) {
            if (((x as i32) + (rect.x as i32)) as u32) == grid.height() || grid.check((((x as i32) + (rect.x as i32)) as u32), (((y as i32) + (rect.y as i32)) as u32)) {
                return true;
            }
        }
        false
    }

    pub fn bounds(&self, y: i32, rotation: Rotation, grid: &graphic::Grid) -> bool {
        for rect in self.rectangles(rotation) {
            if (y + (rect.y as i32)) < 0 || ((y + (rect.y as i32)) as u32) >= grid.width() {
                return true;
            }
        }
        false
    }

    pub fn over(&self, x: u32, rotation: Rotation) -> bool {
        for rect in self.rectangles(rotation) {
            if ((x as i32) + (rect.x as i32)) < 0 {
                return true;
            }
        }
        false
    }

    pub fn random() -> BlockType {
        match rand::thread_rng().gen_range(0, 7) {
            0 => BlockType::T,
            1 => BlockType::I,
            2 => BlockType::O,
            3 => BlockType::J,
            4 => BlockType::L,
            5 => BlockType::S,
            6 => BlockType::Z,
            _ => BlockType::I
        }
    }
}