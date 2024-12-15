use std::io::{self, Result};

use crossterm::{
    event::{self, Event, KeyCode, MouseEvent, MouseEventKind},
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

use crate::game::{Game, HEIGHT, WIDTH};

fn render_board(board: &[Vec<u32>], frame: &mut Frame, area: Rect, title: &str) {
    let canvas = Canvas::default()
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .marker(Marker::Block)
        .paint(|ctx| {
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

pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Ui {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

        Ok(Self { terminal })
    }

    pub fn run(&mut self, mut game: Game) -> Result<()> {
        // Initial UI draw
        self.draw_ui(&game)?;

        loop {
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => {
                            game.select_prev_state();
                            self.draw_ui(&game)?;
                        }
                        KeyCode::Right => {
                            game.select_next_state();
                            self.draw_ui(&game)?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw_ui(&mut self, game: &Game) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // stats
                    Constraint::Min(0),    // boards
                    Constraint::Length(3), // metrics
                    Constraint::Length(3), // stored states navigation
                ])
                .split(area);

            // Stats section
            let lowest_entropy_state = game.lowest_entropy_states.first();
            let stats = format!(
                "Current time: {} | Best entropy: {:.6} at time: {}",
                game.current_time,
                lowest_entropy_state
                    .map(|s| s.entropy)
                    .unwrap_or(f64::INFINITY),
                lowest_entropy_state.map(|s| s.time).unwrap_or(0),
            );
            let stats_widget = Paragraph::new(stats)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(stats_widget, layout[0]);

            // Boards section
            let boards = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(layout[1]);

            // Left panel: Show selected state
            if let Some(state) = game.selected_state() {
                render_board(
                    &state.board,
                    frame,
                    boards[0],
                    &format!("Selected Board (t={})", state.time),
                );
            }

            // Right panel: Always show best board
            if let Some(state) = game.lowest_entropy_states.first() {
                render_board(
                    &state.board,
                    frame,
                    boards[1],
                    &format!("Best Board (t={})", state.time),
                );
            }

            // Metrics section - show metrics for selected state
            let metrics = if let Some(state) = game.selected_state() {
                format!(
                    "Entropy: {:.6} | Safety Factor: {}",
                    state.entropy,
                    game.calculate_safety_factor_for_board(&state.board)
                )
            } else {
                String::from("No state selected")
            };

            let metrics_widget = Paragraph::new(metrics)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(metrics_widget, layout[2]);

            // Navigation list
            let nav_text = game
                .lowest_entropy_states
                .iter()
                .map(|s| {
                    let label = format!("t={:>5}", s.time);
                    if game.is_time_selected(s.time) {
                        format!("[\x1b[34m{} ({:.6})\x1b[0m]", label, s.entropy)
                    } else {
                        format!("[{} ({:.6})]", label, s.entropy)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            let nav_widget = Paragraph::new(nav_text)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Left)
                .wrap(ratatui::widgets::Wrap { trim: true });
            frame.render_widget(nav_widget, layout[3]);
        })?;
        Ok(())
    }

    fn handle_click(&self, game: &mut Game, column: u16, row: u16) {
        let miniature_height = 10;
        let miniature_width = 10;
        let spacing = 2;
        let start_row = self
            .terminal
            .size()
            .expect("Failed to get terminal size")
            .height
            .saturating_sub(miniature_height + 2);

        if row >= start_row {
            let idx = (column as usize) / (miniature_width + spacing);
            if idx < game.lowest_entropy_states.len() {
                game.select_state(idx);
            }
        }
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        io::stdout().execute(LeaveAlternateScreen).ok();
    }
}
