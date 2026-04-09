mod ai;
mod config;
mod scanner;
mod report;
mod notifier;

use std::io;

use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Gauge, Paragraph, Row, Table},
    Frame,
};
use ratatui::Terminal;

use crate::config::Config;
use crate::scanner::ScoutEntry;

// ─── CLI ───────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "atlas", about = "AI-powered CLI for Codebase Scouting and Stakeholder Shipping", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scout local directory tree
    Scout,
    /// Analyze Git diff and contact stakeholders
    Ship,
}

// ─── Animation Frames ──────────────────────────────────────────────────

const LOGO_FRAMES: [&str; 3] = [
    "   (O)   \n  \\_|_/  \n   | |   \n  /   \\  ",
    "  \\(O)/  \n   _|_   \n   | |   \n  /   \\  ",
    "  /(O)\\  \n   _|_   \n   | |   \n  /   \\  ",
];

// ─── App State ─────────────────────────────────────────────────────────

struct App {
    mode: AppMode,
    should_quit: bool,
    config: Option<Config>,
    context_lines: u64,
    context_limit: u64,
    scout_entries: Vec<ScoutEntry>,
    frame_count: usize,
    report: Option<String>,
    is_analyzing: bool,
    pdf_status: Option<String>,
    notification_status: Option<String>,
}

enum AppMode {
    Scout,
    Ship,
}

// ─── Palette ───────────────────────────────────────────────────────────

mod palette {
    use super::*;

    pub const BG: Color = Color::Rgb(2, 6, 23);
    pub const SKY: Color = Color::Rgb(56, 189, 248);
}

// ─── Helpers ───────────────────────────────────────────────────────────

fn bordered_block(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(title, Style::default().fg(palette::SKY).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette::SKY))
        .style(Style::default().bg(palette::BG))
}

// ─── Main ──────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mode = match cli.command {
        Commands::Scout => AppMode::Scout,
        Commands::Ship => AppMode::Ship,
    };

    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Warning: could not load config: {}", e);
            None
        }
    };

    // Perform real scan
    let scout_entries = scanner::scan_directory(".").await;
    let total_lines: usize = scout_entries.iter().map(|e| e.lines).sum();

    let mut app = App {
        mode,
        should_quit: false,
        config,
        context_lines: total_lines as u64,
        context_limit: 5_000,
        scout_entries,
        frame_count: 0,
        report: None,
        is_analyzing: false,
        pdf_status: None,
        notification_status: None,
    };

    setup_terminal()?;
    let mut terminal = create_terminal()?;

    // Channels
    let (tx_ai, mut rx_ai) = tokio::sync::mpsc::channel::<Result<String, String>>(1);
    let (tx_notify, mut rx_notify) = tokio::sync::mpsc::channel::<Result<String, String>>(1);

    while !app.should_quit {
        terminal.draw(|frame| render(frame, &mut app))?;

        // AI results
        if let Ok(result) = rx_ai.try_recv() {
            app.is_analyzing = false;
            match result {
                Ok(report) => app.report = Some(report),
                Err(e) => app.report = Some(format!("Error: {}", e)),
            }
        }

        // Notification results
        if let Ok(result) = rx_notify.try_recv() {
            match result {
                Ok(msg) => app.notification_status = Some(msg),
                Err(e) => app.notification_status = Some(format!("Notify Error: {}", e)),
            }
        }

        // Non-blocking poll
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('c') => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('s') => {
                            app.mode = AppMode::Scout;
                        }
                        KeyCode::Char('h') => {
                            app.mode = AppMode::Ship;
                        }
                        KeyCode::Char('a') => {
                            if !app.is_analyzing {
                                if let Some(ref config) = app.config {
                                    app.is_analyzing = true;
                                    app.report = None;
                                    app.pdf_status = None;
                                    app.notification_status = None;
                                    let api_key = config.api_key.clone();
                                    let context = format!("Codebase with {} files and {} lines of code.", app.scout_entries.len(), app.context_lines);
                                    let tx_clone = tx_ai.clone();
                                    tokio::spawn(async move {
                                        let result = ai::generate_report(&api_key, &context).await;
                                        let _ = tx_clone.send(result).await;
                                    });
                                }
                            }
                        }
                        KeyCode::Char('p') => {
                            if let Some(ref report_text) = app.report {
                                app.pdf_status = Some("Generating PDF...".to_string());
                                match report::generate_pdf(report_text, "atlas_report.pdf") {
                                    Ok(_) => app.pdf_status = Some("PDF saved to atlas_report.pdf".to_string()),
                                    Err(e) => app.pdf_status = Some(format!("PDF Error: {}", e)),
                                }
                            }
                        }
                        KeyCode::Char('n') => {
                            if let Some(ref config) = app.config {
                                if let (Some(sid), Some(token), Some(from)) = (&config.twilio_sid, &config.twilio_token, &config.twilio_from) {
                                    app.notification_status = Some("Sending notification...".to_string());
                                    let sid = sid.clone();
                                    let token = token.clone();
                                    let from = from.clone();
                                    let ceo_phone = config.ceo_phone.clone();
                                    let body = "Project Atlas: Analysis complete. PDF report generated.".to_string();
                                    let tx_clone = tx_notify.clone();
                                    tokio::spawn(async move {
                                        let result = notifier::send_sms(&sid, &token, &from, &ceo_phone, &body).await;
                                        let _ = tx_clone.send(result).await;
                                    });
                                } else {
                                    app.notification_status = Some("Twilio config missing".to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        app.frame_count = app.frame_count.wrapping_add(1);
    }

    restore_terminal()?;
    Ok(())
}

fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn create_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let backend = CrosstermBackend::new(io::stdout());
    Terminal::new(backend)
}

// ─── Render ────────────────────────────────────────────────────────────

fn render(frame: &mut Frame, app: &mut App) {
    // Top-level: dashboard (top) + Status Bar (bottom 3 lines)
    let vertical = Layout::default()
        .constraints([Constraint::Min(6), Constraint::Length(3)])
        .split(frame.area());

    // Split dashboard into Left (35%) and Right (65%)
    let horizontal = Layout::default()
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(vertical[0]);

    // Fill entire frame with deep navy background
    let bg = Block::default().style(Style::default().bg(palette::BG));
    frame.render_widget(bg, frame.area());

    render_left_pane(frame, horizontal[0], app);
    render_right_pane(frame, horizontal[1], app);
    render_status_bar(frame, vertical[1], app);
}

fn render_left_pane(frame: &mut Frame, area: Rect, app: &App) {
    let block = bordered_block(" SYSTEM & CONTEXT ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let inner_layout = Layout::default()
        .constraints([
            Constraint::Length(6), // animated logo
            Constraint::Min(3),    // context density gauge
        ])
        .split(inner);

    // ── Animated ASCII Logo ──
    let frame_idx = app.frame_count % LOGO_FRAMES.len();
    let logo_text = LOGO_FRAMES[frame_idx];
    let logo_para = Paragraph::new(logo_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(palette::SKY).bg(palette::BG));
    frame.render_widget(logo_para, inner_layout[0]);

    // ── Context Density Gauge ──
    let ratio = (app.context_lines as f64 / app.context_limit as f64).min(1.0);
    let gauge = Gauge::default()
        .block(bordered_block(" CONTEXT DENSITY "))
        .gauge_style(Style::default().fg(palette::SKY).bg(Color::Rgb(10, 20, 50)))
        .ratio(ratio)
        .label(Span::styled(
            format!("{} / {}", app.context_lines, app.context_limit),
            Style::default().fg(palette::SKY).add_modifier(Modifier::BOLD),
        ));
    frame.render_widget(gauge, inner_layout[1]);
}

fn render_right_pane(frame: &mut Frame, area: Rect, app: &App) {
    let title = if app.is_analyzing {
        " ENGINE: ANALYZING... "
    } else if app.report.is_some() {
        " ENGINE: REPORT "
    } else {
        " ENGINE: SCOUT "
    };

    let block = bordered_block(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(ref report) = app.report {
        let para = Paragraph::new(report.as_str())
            .wrap(ratatui::widgets::Wrap { trim: true })
            .style(Style::default().fg(Color::Rgb(205, 214, 244)).bg(palette::BG));
        frame.render_widget(para, inner);
    } else {
        match app.mode {
            AppMode::Scout => render_scout_table(frame, inner, app),
            AppMode::Ship => render_ship_panel(frame, inner, app),
        }
    }
}

fn render_scout_table(frame: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec![
        Cell::from(Span::styled(" File Path ", Style::default().fg(palette::BG).add_modifier(Modifier::BOLD))),
        Cell::from(Span::styled(" Size ", Style::default().fg(palette::BG).add_modifier(Modifier::BOLD))),
        Cell::from(Span::styled(" Status ", Style::default().fg(palette::BG).add_modifier(Modifier::BOLD))),
    ]).style(Style::default().fg(palette::SKY));

    let rows: Vec<Row> = app.scout_entries.iter().map(|entry| {
        let size = format!("{} lines", entry.lines);
        Row::new(vec![
            Cell::from(Span::styled(&entry.path, Style::default().fg(Color::Rgb(205, 214, 244)))),
            Cell::from(Span::styled(size, Style::default().fg(palette::SKY))),
            Cell::from(Span::styled(&entry.status, Style::default().fg(palette::SKY).add_modifier(Modifier::BOLD))),
        ])
    }).collect();

    let table = Table::new(rows, [Constraint::Percentage(45), Constraint::Percentage(20), Constraint::Percentage(35)])
        .header(header)
        .column_spacing(2)
        .style(Style::default().bg(palette::BG));

    frame.render_widget(table, area);
}

fn render_ship_panel(frame: &mut Frame, area: Rect, _app: &App) {
    let body = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Ship mode — awaiting git diff trigger.",
            Style::default().fg(palette::SKY),
        )),
        Line::from(""),
    ];
    let para = Paragraph::new(body)
        .alignment(Alignment::Center)
        .style(Style::default().bg(palette::BG));
    frame.render_widget(para, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let mode_label = match app.mode {
        AppMode::Scout => "SCOUT",
        AppMode::Ship => "SHIP",
    };
    let config_status = if app.config.is_some() {
        Span::styled("LINKED", Style::default().fg(palette::SKY).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("NO CONFIG", Style::default().fg(Color::Rgb(150, 150, 170)))
    };

    let mut keys = vec![
        Span::styled(format!(" [{mode_label}] "), Style::default().fg(palette::BG).bg(palette::SKY).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default().bg(palette::BG)),
        Span::styled("[q] Quit", Style::default().fg(palette::SKY)),
        Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))),
        Span::styled("[s] Scout", Style::default().fg(palette::SKY)),
        Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))),
        Span::styled("[h] Ship", Style::default().fg(palette::SKY)),
        Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))),
        Span::styled("[a] Analyze", Style::default().fg(palette::SKY)),
        Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))),
        Span::styled("[n] Notify", Style::default().fg(palette::SKY)),
        Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))),
    ];

    if app.report.is_some() {
        keys.push(Span::styled("[p] PDF", Style::default().fg(palette::SKY)));
        keys.push(Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))));
    }

    if let Some(ref status) = app.pdf_status {
        keys.push(Span::styled(status, Style::default().fg(Color::Rgb(150, 150, 170))));
        keys.push(Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))));
    }

    if let Some(ref status) = app.notification_status {
        keys.push(Span::styled(status, Style::default().fg(Color::Rgb(150, 150, 170))));
        keys.push(Span::styled("  │  ", Style::default().fg(Color::Rgb(40, 50, 80))));
    }

    keys.push(Span::styled("Config: ", Style::default().fg(Color::Rgb(100, 110, 140))));
    keys.push(config_status);

    let bar = Paragraph::new(Line::from(keys))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(palette::SKY))
            .title(Span::styled(" STATUS ", Style::default().fg(palette::SKY).add_modifier(Modifier::BOLD)))
            .style(Style::default().bg(palette::BG)))
        .alignment(Alignment::Center);

    frame.render_widget(bar, area);
}
