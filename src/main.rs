use rand::Rng;

#[derive(Copy, Clone)]
enum Cell {
    Alive,
    Dead,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let cells = vec![vec![Cell::Dead; width]; height];
        Grid { cells }
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < self.cells[0].len() as isize
                    && ny >= 0 && ny < self.cells.len() as isize
                    && matches!(self.cells[ny as usize][nx as usize], Cell::Alive)
                {
                    count += 1;
                }
            }
        }

        count
    }
    fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                let live_neighbors = self.live_neighbor_count(x, y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = new_cells;
    }

    fn display(&self) {
        for row in &self.cells {
            for &cell in row {
                match cell {
                    Cell::Alive => print!("*"),
                    Cell::Dead => print!(" "),
                }
            }
            println!();
        }
        println!("------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = if rng.gen() { // generates a random boolean
                    Cell::Alive
                } else {
                    Cell::Dead
                };
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new(235, 52);
    grid.randomize();
    
    loop {
        grid.display();
        grid.update();

        // Pause for a short duration
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
