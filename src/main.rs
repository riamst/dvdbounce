use std::{
    io::{stdout, Write},
    ops::{Add, Neg, Sub},
    thread::sleep,
    time::Duration,
};

const WIDTH: usize = 128;
const HEIGHT: usize = 32;
const FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS);

#[derive(Debug, Copy, Clone)]
struct Vector(f32, f32);

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.0)
    }
}

fn main() {
    let mut pos = Vector(0 as f32, 0 as f32);
    let mut vel = Vector(0.5 as f32, 0.25 as f32);
    loop {
        pos = pos + vel;
        if pos.0 >= (WIDTH - 11) as f32 || pos.0 <= 0. {
            vel.0 = -vel.0
        }
        if pos.1 >= (HEIGHT - 4) as f32 || pos.1 <= 0. {
            vel.1 = -vel.1
        }
        draw_frame(pos);
    }
}

fn draw_frame(pos: Vector) {
    let x = pos.0.round() as usize;
    let y = pos.1.round() as usize;
    fill(" ");
    back(WIDTH, HEIGHT);
    draw(x, y);
    back(100, y + 5);
    sleep(FRAME_TIME);
}

fn fill(str: &str) {
    print!("{}", format!("{}\n", str.repeat(WIDTH)).repeat(HEIGHT));
    stdout().flush().unwrap();
}

fn back(x: usize, y: usize) {
    print!("\u{1b}[{}D\u{1b}[{}A", x + 30, y);
    stdout().flush().unwrap();
}

fn draw(x: usize, y: usize) {
    let top_pad = "\n".repeat(y);
    let left_pad = " ".repeat(x);
    print!("{}{}", top_pad, left_pad);
    stdout().flush().unwrap();
    print!(
        "####   ####
{0}# # # # # #
{0}# # # # # #
{0}###  #  ###",
        left_pad
    );
    stdout().flush().unwrap();
}
