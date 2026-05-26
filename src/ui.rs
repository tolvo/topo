use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    // Header info (CPU/Mem) placeholder
    let (used_mem, total_mem) = app.system.get_mem_usage();
    let mem_pct = (used_mem as f64 / total_mem as f64) * 100.0;
    
    let header_block = Block::default()
        .title(format!(" Memory: {:.1}GB / {:.1}GB ({:.1}%) ", used_mem as f64 / 1024.0 / 1024.0 / 1024.0, total_mem as f64 / 1024.0 / 1024.0 / 1024.0, mem_pct))
        .borders(Borders::ALL);
    f.render_widget(header_block, chunks[0]);

    // Processes Table
    let header_cells = ["PID", "Name", "CPU%", "Mem (MB)"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(0);

    let rows = app.processes.iter().enumerate().map(|(i, p)| {
        let style = if i == app.selected_index {
            Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        Row::new(vec![
            Cell::from(p.pid.to_string()),
            Cell::from(p.name.clone()),
            Cell::from(format!("{:.1}", p.cpu)),
            Cell::from((p.mem / 1024 / 1024).to_string()),
        ]).style(style)
    });

    let t = Table::new(
        rows,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Processes "))
    .row_highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

    f.render_widget(t, chunks[1]);
}
