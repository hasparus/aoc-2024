pub const WIDTH: usize = 101;
pub const HEIGHT: usize = 103;
pub const UNIQUE_BOARDS: usize = WIDTH * HEIGHT;
pub const MAX_STORED_BOARDS: usize = 100;

#[derive(Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Robot {
    pub position: Position,
    pub velocity: Velocity,
}

#[derive(Clone)]
pub struct BoardState {
    pub board: Vec<Vec<u32>>,
    pub entropy: f64,
    pub time: usize,
}

pub struct Game {
    pub robots: Vec<Robot>,
    pub current_time: usize,
    pub lowest_entropy_states: Vec<BoardState>,
    selected_state_idx: std::cell::Cell<Option<usize>>,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let robots = parse_input(input);
        let mut game = Self {
            robots,
            current_time: 0,
            lowest_entropy_states: Vec::with_capacity(MAX_STORED_BOARDS),
            selected_state_idx: std::cell::Cell::new(None),
        };

        // Precompute all states
        for time in 0..UNIQUE_BOARDS {
            if time % 1000 == 0 {
                print!("\x1B[2J\x1B[1;1H");
                println!("Processing time: {}", time);
            }

            let board = process_time(&game.robots, time);
            let entropy = calculate_entropy(&board);

            if entropy <= 2.7 {
                let state = BoardState {
                    board,
                    entropy,
                    time,
                };
                game.update_lowest_entropy_states(state);
            }
        }

        // Clear the progress messages before starting UI
        print!("\x1B[2J\x1B[1;1H");

        game
    }

    pub fn current_board(&self) -> Vec<Vec<u32>> {
        process_time(&self.robots, self.current_time)
    }

    pub fn calculate_safety_factor_for_board(&self, board: &[Vec<u32>]) -> u32 {
        let mid_x = WIDTH / 2;
        let mid_y = HEIGHT / 2;

        let mut quadrants = [0u32; 4]; // top_left, top_right, bottom_left, bottom_right

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x == mid_x || y == mid_y {
                    continue;
                }

                let count = board[y][x];
                if count == 0 {
                    continue;
                }

                let quad_idx = match (x > mid_x, y > mid_y) {
                    (false, false) => 0, // top left
                    (true, false) => 1,  // top right
                    (false, true) => 2,  // bottom left
                    (true, true) => 3,   // bottom right
                };

                quadrants[quad_idx] += count;
            }
        }

        quadrants.iter().product()
    }

    pub fn calculate_safety_factor(&self) -> u32 {
        self.calculate_safety_factor_for_board(&self.current_board())
    }

    pub fn current_entropy(&self) -> f64 {
        calculate_entropy(&self.current_board())
    }

    pub fn select_state(&self, idx: usize) {
        if idx < self.lowest_entropy_states.len() {
            self.selected_state_idx.set(Some(idx));
        }
    }

    pub fn selected_state(&self) -> Option<&BoardState> {
        self.selected_state_idx
            .get()
            .and_then(|idx| self.lowest_entropy_states.get(idx))
    }

    pub fn select_next_state(&self) {
        let current_idx = self.selected_state_idx.get().unwrap_or(0);
        if current_idx + 1 < self.lowest_entropy_states.len() {
            self.select_state(current_idx + 1);
        }
    }

    pub fn select_prev_state(&self) {
        let current_idx = self.selected_state_idx.get().unwrap_or(0);
        if current_idx > 0 {
            self.select_state(current_idx - 1);
        }
    }

    fn update_lowest_entropy_states(&mut self, state: BoardState) {
        let insert_pos = self
            .lowest_entropy_states
            .binary_search_by(|probe| probe.entropy.partial_cmp(&state.entropy).unwrap())
            .unwrap_or_else(|pos| pos);

        if insert_pos < MAX_STORED_BOARDS {
            self.lowest_entropy_states.insert(insert_pos, state);
            if self.lowest_entropy_states.len() > MAX_STORED_BOARDS {
                self.lowest_entropy_states.pop();
            }
        }
    }

    pub fn is_current_selected(&self) -> bool {
        self.selected_state_idx.get().is_none()
    }

    pub fn is_time_selected(&self, time: usize) -> bool {
        self.selected_state()
            .map_or(false, |state| state.time == time)
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line
                .split(['p', 'v', '='])
                .filter(|s| !s.trim().is_empty())
                .collect();

            let position: Vec<i32> = parts[0]
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            let velocity: Vec<i32> = parts[1]
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            Robot {
                position: Position {
                    x: position[0],
                    y: position[1],
                },
                velocity: Velocity {
                    x: velocity[0],
                    y: velocity[1],
                },
            }
        })
        .collect()
}

fn calculate_entropy(board: &[Vec<u32>]) -> f64 {
    let total_cells = (WIDTH * HEIGHT) as f64;
    let occupied_cells = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell > 0)
        .count() as f64;

    let p1 = occupied_cells / total_cells;
    let p2 = 1.0 - p1;

    let e1 = if p1 == 0.0 { 0.0 } else { -p1 * p1.log2() };
    let e2 = if p2 == 0.0 { 0.0 } else { -p2 * p2.log2() };

    e1 + e2
}

fn process_time(robots: &[Robot], time: usize) -> Vec<Vec<u32>> {
    let mut current_robots = robots.to_vec();
    let mut board = vec![vec![0; WIDTH]; HEIGHT];

    for _ in 0..time {
        for robot in &mut current_robots {
            robot.position.x = (robot.position.x + robot.velocity.x + WIDTH as i32) % WIDTH as i32;
            robot.position.y =
                (robot.position.y + robot.velocity.y + HEIGHT as i32) % HEIGHT as i32;
        }
    }

    for robot in current_robots {
        let x = robot.position.x as usize;
        let y = robot.position.y as usize;
        board[y][x] += 1;
    }

    board
}
