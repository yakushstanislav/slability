use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::layout::Rect;
use tui::style::Modifier;
use tui::text::Spans;
use tui::widgets::Paragraph;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame, Terminal,
};

use time::format_description;

use crate::monitor::Monitor;

pub struct Context {
    start_time: Instant,
}

impl Context {
    pub fn new() -> Self {
        Context {
            start_time: Instant::now(),
        }
    }
}

pub fn initialize() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    Ok(Terminal::new(backend)?)
}

pub fn destroy(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

pub fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    context: &Context,
    monitors: &Vec<Monitor>,
) -> io::Result<()> {
    const TICK_RATE: Duration = Duration::from_secs(1);

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| draw_ui(frame, context, monitors))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }
}

fn draw_ui<B: Backend>(frame: &mut Frame<B>, context: &Context, monitors: &Vec<Monitor>) {
    let size = frame.size();

    let body = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(size);

    draw_ui_header(frame, body[0]);
    draw_ui_body(frame, body[1], monitors);
    draw_ui_footer(frame, body[2], context);
}

fn draw_ui_header<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let header = Block::default()
        .borders(Borders::BOTTOM)
        .title(vec![Span::styled(
            "slability",
            Style::default().fg(Color::Red),
        )])
        .title_alignment(Alignment::Center);

    frame.render_widget(header, area);
}

fn draw_ui_body<B: Backend>(frame: &mut Frame<B>, area: Rect, monitors: &Vec<Monitor>) {
    let constraints: Vec<_> = monitors.iter().map(|_| Constraint::Length(3)).collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(area);

    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

    for (index, monitor) in monitors.iter().enumerate() {
        let text = Spans::from(vec![
            Span::styled("IP: ", Style::default().fg(Color::Blue)),
            Span::styled(
                monitor.address().to_string(),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(" "),
            Span::styled(
                format!(
                    "[{}]",
                    match monitor.get_state().is_online() {
                        Some(true) => "ONLINE",
                        Some(false) => "OFFLINE",
                        None => "WAIT",
                    }
                ),
                Style::default().fg(match monitor.get_state().is_online() {
                    Some(true) => Color::Green,
                    Some(false) => Color::Red,
                    None => Color::DarkGray,
                }),
            ),
            Span::raw(" "),
            Span::styled(
                format!("RESTARTS: {}", monitor.get_state().restarts()),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" "),
            Span::styled(
                format!(
                    "ON_TIME: {} [{}s]",
                    match monitor.get_state().last_online() {
                        Some(time) => time.format(&format).unwrap().to_string(),
                        None => "N/A".to_string(),
                    },
                    match monitor.get_state().elapsed_online() {
                        Some(time) => time.elapsed().as_secs().to_string(),
                        None => 0.to_string(),
                    }
                ),
                Style::default().fg(Color::LightCyan),
            ),
            Span::raw(" "),
            Span::styled(
                format!(
                    "OFF_TIME: {} [{}s]",
                    match monitor.get_state().last_offline() {
                        Some(time) => time.format(&format).unwrap().to_string(),
                        None => "N/A".to_string(),
                    },
                    match monitor.get_state().elapsed_offline() {
                        Some(time) => time.elapsed().as_secs().to_string(),
                        None => 0.to_string(),
                    }
                ),
                Style::default().fg(Color::LightRed),
            ),
        ]);

        let block = Block::default()
            .title(vec![Span::styled(
                monitor.name(),
                Style::default().fg(Color::Blue),
            )])
            .title_alignment(Alignment::Center);

        let paragraph = Paragraph::new(text).block(block);

        frame.render_widget(paragraph, chunks[index]);
    }
}

fn draw_ui_footer<B: Backend>(frame: &mut Frame<B>, area: Rect, context: &Context) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let style = Style::default().fg(Color::DarkGray);

    let quit = Paragraph::new(Spans::from(vec![Span::raw("Press 'q' to quit.")]))
        .style(style)
        .alignment(Alignment::Left)
        .wrap(tui::widgets::Wrap { trim: true });

    frame.render_widget(quit, chunks[0]);

    let elapsed = Paragraph::new(Spans::from(vec![Span::raw(format!(
        "Elapsed: {}s",
        context.start_time.elapsed().as_secs()
    ))]))
    .style(style.add_modifier(Modifier::BOLD))
    .alignment(Alignment::Right)
    .wrap(tui::widgets::Wrap { trim: true });

    frame.render_widget(elapsed, chunks[1]);
}
