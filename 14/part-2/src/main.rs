use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::{fs, io};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const UNIQUE_BOARDS: usize = WIDTH * HEIGHT;

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
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
                .map(|s| s.trim().parse().expect("Failed to parse position"))
                .collect();

            let velocity: Vec<i32> = parts[1]
                .split(',')
                .map(|s| s.trim().parse().expect("Failed to parse velocity"))
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

fn calculate_entropy(board: &Vec<Vec<u32>>) -> f64 {
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

fn render_board(board: &[Vec<u32>], frame: &mut Frame, area: Rect, title: &str) {
    let canvas = Canvas::default()
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .marker(Marker::Block)
        .paint(|ctx| {
            let midx = WIDTH as f64 / 2.0;
            let midy = HEIGHT as f64 / 2.0;

            // Draw middle lines
            ctx.draw(&CanvasLine {
                x1: midx,
                y1: 0.0,
                x2: midx,
                y2: HEIGHT as f64,
                color: Color::Red,
            });
            ctx.draw(&CanvasLine {
                x1: 0.0,
                y1: midy,
                x2: WIDTH as f64,
                y2: midy,
                color: Color::Red,
            });

            // Draw robots
            for (y, row) in board.iter().enumerate() {
                for (x, &count) in row.iter().enumerate() {
                    if count > 0 {
                        ctx.print(x as f64, y as f64, "■");
                    }
                }
            }
        });

    frame.render_widget(canvas, area);
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let input = fs::read_to_string("../input.txt").expect("Failed to read input file");
    let robots = parse_input(&input);

    let mut lowest_entropy = f64::INFINITY;
    let mut lowest_entropy_time = 0;
    let mut lowest_entropy_board = vec![vec![0; WIDTH]; HEIGHT];
    let mut current_time = 0;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            // Create layout
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Stats
                    Constraint::Min(0),    // Boards
                ])
                .split(area);

            // Stats at the top
            let stats = format!(
                "Current time: {} | Lowest entropy: {:.6} at time: {}",
                current_time, lowest_entropy, lowest_entropy_time
            );
            let stats_widget = Paragraph::new(stats)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(stats_widget, layout[0]);

            // Split bottom area for two boards
            let boards = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(layout[1]);

            let current_board = process_time(&robots, current_time);
            render_board(&current_board, frame, boards[0], "Current Board");
            render_board(
                &lowest_entropy_board,
                frame,
                boards[1],
                "Lowest Entropy Board",
            );
        })?;

        if current_time >= UNIQUE_BOARDS {
            break;
        }

        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        let board = process_time(&robots, current_time);
        let entropy = calculate_entropy(&board);

        if entropy < lowest_entropy {
            lowest_entropy = entropy;
            lowest_entropy_time = current_time;
            lowest_entropy_board = board;
        }

        current_time += 1;
    }

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
