extern crate rand;

use rand::Rng;
use std::time::{Duration};
use std::thread::sleep;

struct Board {
    width: i32,
    height: i32,
    data: Vec<i8>
}

impl Board {
    fn new(w: i32, h: i32) -> Board {
        let mut data: Vec<i8> = Vec::new();
        for _ in (0..(w*h)) {
            data.push(rand::thread_rng().gen_range(0,2));
        }
        Board {
            width: w,
            height: h,
            data: data
        }
    }

    fn show(&self) {
        for idx in (0..(self.width * self.height)) {
            let datum = self.data[idx as usize];
            if datum == 0 {
                print!(" ");
            } else if datum == 1 {
                print!("●");
            }
            if (idx + 1) % self.width == 0 {
                println!("");
            }
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> i8 {

        let new_x = (x + self.width) % self.width;
        let new_y = (y + self.height) % self.height;

        let idx = new_x + new_y * self.width;
        return self.data[idx as usize];
    }

    fn calc_next_cell(&self, x: i32, y: i32) -> i8 {
        let mut neighbors = 0;
        neighbors += self.get_cell(x - 1, y - 1);
        neighbors += self.get_cell(x - 1, y);
        neighbors += self.get_cell(x - 1, y + 1);

        neighbors += self.get_cell(x, y - 1);
        neighbors += self.get_cell(x, y + 1);

        neighbors += self.get_cell(x + 1, y - 1);
        neighbors += self.get_cell(x + 1, y);
        neighbors += self.get_cell(x + 1, y + 1);

        if self.get_cell(x, y) == 0 {
            if neighbors == 3 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if neighbors == 2 || neighbors == 3 {
                return 1;
            } else {
                return 0;
            }
        }
    }

    fn update_board(&self) -> Board {
        let mut new_data: Vec<i8> = Vec::new();
        for idx in (0..(self.width * self.height)) {
            let x = idx % self.width;
            let y = idx / self.height;
            new_data.push(self.calc_next_cell(x, y));
        }
        Board {
            width: self.width,
            height: self.height,
            data: new_data
        }
    }
}

fn main() {
    let mut board = Board::new(30, 30);
    board.show();
    for _ in (0..100) {
        board = board.update_board();
        println!("\r=============================================");
        board.show();

        sleep(Duration::from_millis(500));
    }
}
