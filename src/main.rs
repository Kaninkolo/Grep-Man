use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use regex::Regex;
use std::io::stdout;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "gman")]
#[command(about = "Search man pages and jump to specific lines", long_about = None)]
#[command(disable_help_flag = true)]
#[command(disable_version_flag = true)]
struct Args {
    /// Show help information
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Show version information
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,

    /// Case sensitive search
    #[arg(short, long)]
    case_sensitive: bool,

    /// Program to search the man page for
    program: String,

    /// Search term to find in the man page (if omitted, opens the man page directly)
    #[arg(allow_hyphen_values = true)]
    term: Option<String>,
}

#[derive(Clone)]
struct Match {
    line_number: usize,
    content: String,
    context_before: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // If no search term provided, just open the man page
    if args.term.is_none() {
        open_man_page(&args.program)?;
        return Ok(());
    }

    let term = args.term.unwrap();

    // Extract man page text
    let man_text = extract_man_page(&args.program)?;

    // Search for matches
    let matches = search_man_page(&man_text, &term, args.case_sensitive);

    if matches.is_empty() {
        println!("No matches found for '{}' in man page for '{}'", term, args.program);
        return Ok(());
    }

    // Show interactive selection menu
    let selected = show_selection_menu(&matches, &args.program, &term)?;

    // Jump to selected line in man page
    if let Some(match_item) = selected {
        jump_to_line(&args.program, match_item.line_number)?;
    }

    Ok(())
}

fn extract_man_page(program: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("man")
        .arg("-P")
        .arg("cat")
        .arg(program)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to get man page for '{}'", program).into());
    }

    let raw_text = String::from_utf8_lossy(&output.stdout);

    // Strip control characters (backspaces used for bold/underline)
    let clean_text = strip_control_chars(&raw_text);

    Ok(clean_text)
}

fn strip_control_chars(text: &str) -> String {
    // Remove backspace sequences like 'a\x08a' used for bold
    let re = Regex::new(r".\x08").unwrap();
    let cleaned = re.replace_all(text, "");

    // Remove other control characters except newlines and tabs
    cleaned
        .chars()
        .filter(|c| *c == '\n' || *c == '\t' || !c.is_control())
        .collect()
}

fn search_man_page(text: &str, term: &str, case_sensitive: bool) -> Vec<Match> {
    let lines: Vec<&str> = text.lines().collect();
    let mut matches = Vec::new();

    let search_term = if case_sensitive {
        term.to_string()
    } else {
        term.to_lowercase()
    };

    for (i, line) in lines.iter().enumerate() {
        let search_line = if case_sensitive {
            line.to_string()
        } else {
            line.to_lowercase()
        };

        if search_line.contains(&search_term) {
            let context_before = if i > 0 {
                Some(lines[i - 1].to_string())
            } else {
                None
            };

            matches.push(Match {
                line_number: i + 1, // 1-indexed for display
                content: line.to_string(),
                context_before,
            });
        }
    }

    matches
}

fn show_selection_menu(
    matches: &[Match],
    program: &str,
    term: &str,
) -> Result<Option<Match>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let result = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new(format!(
                "Found {} matches for '{}' in '{}' man page",
                matches.len(),
                term,
                program
            ))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("gman"));

            f.render_widget(header, chunks[0]);

            // Match list
            let items: Vec<ListItem> = matches
                .iter()
                .map(|m| {
                    let mut lines = Vec::new();

                    // Show context line if available
                    if let Some(ref context) = m.context_before {
                        lines.push(Line::from(vec![
                            Span::styled(
                                format!("  {:4} ", m.line_number - 1),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled(
                                truncate(context, 100),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]));
                    }

                    // Show the matching line
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("▶ {:4} ", m.line_number),
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(truncate(&m.content, 100)),
                    ]));

                    ListItem::new(lines)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Matches"))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut list_state);

            // Footer
            let footer = Paragraph::new("↑/↓: Navigate | Enter: Jump to line | q/Esc: Quit")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));

            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break None,
                    KeyCode::Down | KeyCode::Char('j') => {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i >= matches.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    matches.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(i) = list_state.selected() {
                            break Some(matches[i].clone());
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(result)
}

fn open_man_page(program: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("man")
        .arg(program)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

fn jump_to_line(program: &str, line_number: usize) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("man")
        .arg("-P")
        .arg(format!("less +{}G", line_number))
        .arg(program)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}
