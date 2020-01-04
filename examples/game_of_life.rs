use game_loop::game_loop;
use std::cell::RefCell;

fn main() {
    let game = GameOfLife::new(12, 12);

    // Make some of the cells alive.
    game.set(5, 5);
    game.set(5, 6);
    game.set(5, 7);
    game.set(6, 6);

    // Run the game loop with 2 updates per second.
    let g = game_loop(2, |_| {
        game.update();
    }, |g| {
        // Pass the blending factor (even though this example doesn't use it).
        game.render(g.blending_factor());

        // Exit after 10 seconds.
        if g.running_time() > 10.0 {
            g.exit();
        }
    });

    // Use the 'g' variable to query the game loop after it finishes.
    println!("Exiting after {} seconds", g.running_time());
    println!("");
    println!("Number of updates: {}", g.number_of_updates());
    println!("Number of renders: {}", g.number_of_renders());
}

// A quick and dirty implementation of Conway's Game of Life.
// This uses RefCell so it can be used in both the update and render closure.

struct GameOfLife {
    board: RefCell<Board>,
    width: usize,
    height: usize,
}

type Board = Vec<Vec<bool>>;

impl GameOfLife {
    fn new(width: usize, height: usize) -> Self {
        let board = RefCell::new(vec![vec![false; width]; height]);

        Self { board, width, height }
    }

    fn set(&self, x: usize, y: usize) {
        (*self.board.borrow_mut())[y][x] = true;
    }

    fn update(&self) {
        *self.board.borrow_mut() = self.next_board();
    }

    fn next_board(&self) -> Board {
        (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                self.next_cell(x, y)
            }).collect()
        }).collect()
    }

    fn next_cell(&self, x: usize, y: usize) -> bool {
        let cell = self.board.borrow()[y][x];
        let count = self.alive_neighbors(x as isize, y as isize);

        count == 3 || (cell && count == 2)
    }

    fn alive_neighbors(&self, x: isize, y: isize) -> usize {
        self.neighbors(x, y).iter().filter(|b| **b).count()
    }

    fn neighbors(&self, x: isize, y: isize) -> [bool; 8] {
        [
            self.neighbor(x - 1, y - 1), // top left
            self.neighbor(x    , y - 1), // above
            self.neighbor(x + 1, y - 1), // top right
            self.neighbor(x - 1, y    ), // left
            self.neighbor(x + 1, y    ), // right
            self.neighbor(x - 1, y + 1), // bottom left
            self.neighbor(x    , y + 1), // below
            self.neighbor(x + 1, y + 1), // bottom right
        ]

    }

    fn neighbor(&self, x: isize, y: isize) -> bool {
        if x < 0 { return false; }
        if y < 0 { return false; }

        *self.board.borrow()
            .get(y as usize).unwrap_or(&vec![])
            .get(x as usize).unwrap_or(&false)
    }

    fn render(&self, _blending_factor: f64) {
        print!("{}[2J", 27 as char); // clear terminal

        for row in self.board.borrow().iter() {
            for cell in row {
                if *cell {
                    print!("X");
                } else {
                    print!("_");
                }
            }
            println!();
        }
        println!();

        // Limit rendering to a maximum of 10 frames per second.
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
