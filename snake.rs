#![feature(lang_items)]
#![no_std]

use base::prelude::*;

mod base;
mod gba;

enum Tile { Empty, Snake, Food }
static WIDTH: uint = 30;
static HEIGHT: uint = 20;
static MAX_LENGTH: uint = 100;

struct Arena {
    data: [Tile; WIDTH*HEIGHT]
}

impl Arena {
    pub fn new() -> Arena { Arena { data: [Empty; WIDTH*HEIGHT] } }

    pub fn set(&mut self, x: uint, y: uint, tile: Tile) {
        if x < WIDTH && y < HEIGHT {
            self.data[x + y * WIDTH] = tile;
            let bg_tile = match tile {
                Empty => 1u16,
                Snake => 0u16,
                Food => 1u16 << 12
            };
            gba::hw::write_vram16((0x400usize + x + y * 32) as u32, bg_tile);
        }
    }

    pub fn get(&self, x: uint, y: uint) -> Tile {
        if x < WIDTH && y < HEIGHT {
            self.data[x + y * WIDTH]
        } else { Snake }
    }
}

struct Pos { x: uint, y: uint }

enum Dir { Up, Down, Left, Right }

struct Game {
    arena: Arena,
    pos: Pos,
    snake: [Pos; MAX_LENGTH],
    length: uint,
    target_length: uint,
    dir: Dir,
    rand: Rand,
    food_count: uint
}

impl Game {
    fn new() -> Game {
        Game {
            arena: Arena::new(),
            pos: Pos { x: 15, y: 12 },
            snake: [Pos { x: 0, y: 0 }; MAX_LENGTH],
            length: 0,
            target_length: 5,
            dir: Up,
            rand: Rand::new(1234),
            food_count: 0
        }
    }
    fn reset(&mut self) {
        for y in range(0usize, 24usize) {
            for x in range(0usize, 30usize) {
                self.arena.set(x, y, Empty);
            }
        }
        self.pos.x = WIDTH / 2;
        self.pos.y = HEIGHT / 2;
        self.length = 0;
        self.target_length = 5;
        self.dir = Up;
        self.food_count = 0;
        self.arena.set(self.pos.x, self.pos.y, Snake);
    }

    fn update(&mut self, key_state: &gba::KeyState) {
        if key_state.is_triggered(gba::KeyUp) { self.dir = Up }
        if key_state.is_triggered(gba::KeyDown) { self.dir = Down }
        if key_state.is_triggered(gba::KeyLeft) { self.dir = Left }
        if key_state.is_triggered(gba::KeyRight) { self.dir = Right }
        self.snake[self.length].x = self.pos.x;
        self.snake[self.length].y = self.pos.y;
        if self.length < self.target_length {
            self.length += 1;
        } else {
            self.arena.set(self.snake[0].x, self.snake[0].y, Empty);
            for i in range(0usize, self.length) {
                self.snake[i].x = self.snake[i + 1].x;
                self.snake[i].y = self.snake[i + 1].y;
            }
        }
        let food_x = (self.rand.next_u8() & 31) as uint;
        let food_y = (self.rand.next_u8() & 31) as uint;
        if self.food_count < 4 && food_x < WIDTH && food_y < HEIGHT {
            match self.arena.get(food_x, food_y) {
                Empty => {
                    self.arena.set(food_x, food_y, Food);
                    self.food_count += 1;
                }
                _ => {}
            }
        }
        match self.dir {
            Up => { self.pos.y -= 1 }
            Down => { self.pos.y += 1 }
            Left => { self.pos.x -= 1 }
            Right => { self.pos.x += 1 }
        }
        match self.arena.get(self.pos.x, self.pos.y) {
            Snake => self.reset(),
            Food => {
                self.food_count -= 1;
                self.target_length += 5;
                if self.target_length > MAX_LENGTH - 1 {
                    self.target_length = MAX_LENGTH - 1;
                }
            }
            _ => {}
        };
        self.arena.set(self.pos.x, self.pos.y, Snake);
    }
}

#[start]
pub fn main(_: int, _: *const *const u8) -> int {
    let mut key_state = gba::KeyState::new();
    gba::hw::write_dispcnt(1 << 8);
    gba::hw::write_bg0cnt(1 << 8);
    gba::hw::write_pal(15, 0x7fff);
    gba::hw::write_pal(31, 31 << 5);
    for i in range(1u32, 7u32) {
        gba::hw::write_vram16(i * 2, 0xfff0);
        gba::hw::write_vram16(i * 2 + 1, 0x0fff);
    }
    let mut game = Game::new();
    game.reset();
    loop {
        key_state.update();
        game.update(&key_state);
        for _ in range(0, 4) {
            gba::wait_vblank();
        }
    }
}
