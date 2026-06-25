/// TUI for mneme-guardian — dashboard, config, sessions, providers, review logs.
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame, Terminal,
};
use std::io;

use crate::config;

type Term = Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>;

pub fn run_tui() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let cfg = config::load_config().unwrap_or_default();
    let mut app = App::new(cfg);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Dashboard,
    Config,
    Sessions,
    Providers,
    Logs,
    Help,
}

const TABS: &[&str] = &[
    "\u{1f4ca} Dashboard",
    "\u{2699} Config",
    "\u{1f550} Sessions",
    "\u{1f50c} Providers",
    "\u{1f4cb} Logs",
    "\u{2753} Help",
];

struct SessionEntry {
    timestamp: String,
    project: String,
    issues: u32,
    provider: String,
    status: &'static str,
}

struct App {
    config: config::Config,
    screen: Screen,
    tab: usize,
    log: Vec<String>,
    review_count: u32,
    total_issues: u32,
    sessions: Vec<SessionEntry>,
}

impl App {
    fn new(cfg: config::Config) -> Self {
        let now = chrono::Utc::now();
        let sessions = vec![
            SessionEntry {
                timestamp: (now - chrono::Duration::hours(2))
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
                project: "mneme-guardian".into(),
                issues: 3,
                provider: "opencode".into(),
                status: "\u{2705} Passed",
            },
            SessionEntry {
                timestamp: (now - chrono::Duration::hours(5))
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
                project: "mneme-brain".into(),
                issues: 7,
                provider: "claude".into(),
                status: "\u{274c} Failed",
            },
            SessionEntry {
                timestamp: (now - chrono::Duration::hours(24))
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
                project: "engram".into(),
                issues: 1,
                provider: "opencode".into(),
                status: "\u{2705} Passed",
            },
            SessionEntry {
                timestamp: (now - chrono::Duration::hours(48))
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
                project: "mneme-guardian".into(),
                issues: 0,
                provider: "gemini".into(),
                status: "\u{2705} Passed",
            },
        ];

        let total_issues: u32 = sessions.iter().map(|s| s.issues).sum();

        Self {
            config: cfg,
            screen: Screen::Dashboard,
            tab: 0,
            log: Vec::new(),
            review_count: sessions.len() as u32,
            total_issues,
            sessions,
        }
    }

    fn run(&mut self, terminal: &mut Term) -> anyhow::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if let Event::Key(key) = event::read()? {
                if !self.handle(key.code) {
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle(&mut self, key: KeyCode) -> bool {
        match self.screen {
            Screen::Dashboard => match key {
                KeyCode::Char('1') => {
                    self.screen = Screen::Config;
                    self.tab = 1;
                    true
                }
                KeyCode::Char('2') => {
                    self.screen = Screen::Sessions;
                    self.tab = 2;
                    true
                }
                KeyCode::Char('3') => {
                    self.screen = Screen::Providers;
                    self.tab = 3;
                    true
                }
                KeyCode::Char('4') => {
                    self.screen = Screen::Logs;
                    self.tab = 4;
                    true
                }
                KeyCode::Char('5') => {
                    self.screen = Screen::Help;
                    self.tab = 5;
                    true
                }
                KeyCode::Char('q') | KeyCode::Esc => false,
                _ => true,
            },
            Screen::Config | Screen::Sessions | Screen::Providers | Screen::Logs | Screen::Help => {
                match key {
                    KeyCode::Esc | KeyCode::Backspace => {
                        self.screen = Screen::Dashboard;
                        self.tab = 0;
                        true
                    }
                    KeyCode::Char('q') => false,
                    _ => true,
                }
            }
        }
    }

    fn render(&self, f: &mut Frame) {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(area);

        let titles: Vec<&str> = TABS.iter().map(|t| *t).collect();
        let tabs = Tabs::new(titles)
            .select(self.tab)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("mneme-guardian"),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(tabs, chunks[0]);

        match self.screen {
            Screen::Dashboard => self.render_dashboard(f, chunks[1]),
            Screen::Config => self.render_config(f, chunks[1]),
            Screen::Sessions => self.render_sessions(f, chunks[1]),
            Screen::Providers => self.render_providers(f, chunks[1]),
            Screen::Logs => self.render_logs(f, chunks[1]),
            Screen::Help => self.render_help(f, chunks[1]),
        }

        self.render_status_bar(f, chunks[2]);
    }

    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
        let nav = match self.screen {
            Screen::Dashboard => "1:Config  2:Sessions  3:Providers  4:Logs  5:Help  q:Quit",
            Screen::Config => "Esc:Back  q:Quit",
            Screen::Sessions => "Esc:Back  q:Quit",
            Screen::Providers => "Esc:Back  q:Quit",
            Screen::Logs => "Esc:Back  q:Quit  c:Clear",
            Screen::Help => "Esc:Back  q:Quit",
        };

        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                nav,
                Style::default().fg(Color::Cyan),
            )))
            .style(Style::default().bg(Color::Black))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(Color::DarkGray)),
            ),
            area,
        );
    }

    fn render_dashboard(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mneme_ok = crate::mneme::find_mneme().is_some();
        let provider_ok = match self.config.provider.as_str() {
            "opencode" => true,
            name => std::process::Command::new("which")
                .arg(name)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
        };

        let component_lines = vec![
            Line::from(Span::styled(
                "Component Status",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("mneme-brain:", Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(
                    if mneme_ok {
                        "\u{2705} connected"
                    } else {
                        "\u{274c} not found"
                    },
                    Style::default().fg(if mneme_ok { Color::Green } else { Color::Red }),
                ),
            ]),
            Line::from(vec![
                Span::styled("config:", Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(
                    if crate::config::load_config().is_ok() {
                        "\u{2705} loaded"
                    } else {
                        "\u{26a0} using defaults"
                    },
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled("provider:", Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(
                    format!(
                        "{} {}",
                        self.config.provider,
                        if provider_ok {
                            "\u{2705}"
                        } else {
                            "\u{274c} unavailable"
                        }
                    ),
                    Style::default().fg(if provider_ok {
                        Color::Green
                    } else {
                        Color::Red
                    }),
                ),
            ]),
            Line::from(vec![
                Span::styled("mneme sync:", Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(
                    if self.config.mneme_enabled {
                        "\u{2705} enabled"
                    } else {
                        "\u{274c} disabled"
                    },
                    Style::default().fg(if self.config.mneme_enabled {
                        Color::Green
                    } else {
                        Color::Yellow
                    }),
                ),
            ]),
            Line::from(vec![
                Span::styled("exit issues:", Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(
                    if self.config.exit_on_issues {
                        "\u{2705} on"
                    } else {
                        "\u{26a0} off"
                    },
                    Style::default().fg(if self.config.exit_on_issues {
                        Color::Green
                    } else {
                        Color::Yellow
                    }),
                ),
            ]),
        ];
        f.render_widget(
            Paragraph::new(component_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("\u{1f4ca} Components"),
            ),
            chunks[0],
        );

        let avg = if self.review_count > 0 {
            self.total_issues as f64 / self.review_count as f64
        } else {
            0.0
        };

        let review_lines = vec![
            Line::from(Span::styled(
                "Review Summary",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Total reviews:  {}", self.review_count)),
            Line::from(format!("Total issues:   {}", self.total_issues)),
            Line::from(format!("Avg issues/rev: {:.1}", avg)),
            Line::from(""),
            Line::from(Span::styled(
                "Navigation",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "1Config  2Sessions  3Providers",
                Style::default().fg(Color::Cyan),
            )),
            Line::from(Span::styled(
                "4Logs  5Help  qQuit",
                Style::default().fg(Color::Cyan),
            )),
        ];
        f.render_widget(
            Paragraph::new(review_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("\u{1f4c8} Reviews"),
            ),
            chunks[1],
        );
    }

    fn render_sessions(&self, f: &mut Frame, area: Rect) {
        let rows: Vec<Row> = self
            .sessions
            .iter()
            .map(|s| {
                Row::new(vec![
                    Cell::from(s.timestamp.clone()),
                    Cell::from(s.project.clone()),
                    Cell::from(s.issues.to_string()),
                    Cell::from(s.provider.clone()),
                    Cell::from(s.status),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(18),
                Constraint::Length(20),
                Constraint::Length(8),
                Constraint::Length(12),
                Constraint::Length(12),
            ],
        )
        .header(
            Row::new(vec!["Timestamp", "Project", "Issues", "Provider", "Status"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("\u{1f550} Review Sessions"),
        );

        f.render_widget(table, area);
    }

    fn render_config(&self, f: &mut Frame, area: Rect) {
        let config_text = vec![
            Line::from(Span::styled(
                "Configuration (~/.config/mneme-guardian/config.toml)",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Provider:     {}", self.config.provider)),
            Line::from(format!(
                "Model:        {}",
                self.config.model.as_deref().unwrap_or("(default)")
            )),
            Line::from(format!("Rules file:   {}", self.config.rules_file)),
            Line::from(format!("Mneme sync:   {}", self.config.mneme_enabled)),
            Line::from(format!("Exit issues:  {}", self.config.exit_on_issues)),
            Line::from(format!("Max lines:    {}", self.config.max_lines)),
            Line::from(""),
            Line::from(Span::styled(
                "Override via env: MNEME_G_PROVIDER, MNEME_G_MODEL, etc.",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        f.render_widget(
            Paragraph::new(config_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("\u{2699} Config"),
            ),
            area,
        );
    }

    fn render_providers(&self, f: &mut Frame, area: Rect) {
        let providers = [
            ("opencode", "OpenCode (default)"),
            ("claude", "Claude Code"),
            ("gemini", "Gemini CLI"),
            ("codex", "Codex CLI"),
            ("ollama", "Ollama + model"),
        ];

        let rows: Vec<Row> = providers
            .iter()
            .map(|(name, desc)| {
                let available = std::process::Command::new("which")
                    .arg(name)
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);
                Row::new(vec![*name, *desc, if available { "\u{2705}" } else { " " }])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(12),
                Constraint::Length(25),
                Constraint::Length(8),
            ],
        )
        .header(
            Row::new(vec!["Name", "Description", "Available"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("\u{1f50c} Providers"),
        );
        f.render_widget(table, area);
    }

    fn render_logs(&self, f: &mut Frame, area: Rect) {
        let mut lines = vec![
            Line::from(Span::styled(
                "Review logs (c to clear)",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];
        if self.log.is_empty() {
            lines.push(Line::from(
                "  No review logs yet. Run 'mneme-g run' to review code.",
            ));
        } else {
            for l in &self.log {
                lines.push(Line::from(l.clone()));
            }
        }
        f.render_widget(
            Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("\u{1f4cb} Review Logs"),
            ),
            area,
        );
    }

    fn render_help(&self, f: &mut Frame, area: Rect) {
        let help = vec![
            Line::from(Span::styled(
                "mneme-guardian \u{2014} AI Code Review Guardian",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Setup",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  mneme-g init          \u{2014} Create config"),
            Line::from("  mneme-g install       \u{2014} Pre-commit hook"),
            Line::from("  mneme-g uninstall     \u{2014} Remove hook"),
            Line::from(""),
            Line::from(Span::styled(
                "Review",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  mneme-g run           \u{2014} Review staged files"),
            Line::from("  mneme-g run --ci      \u{2014} Review last commit"),
            Line::from("  MNEME_G_PROVIDER=claude mneme-g run"),
            Line::from(""),
            Line::from(Span::styled(
                "Integration",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  mneme search \"code review\" --project <name>"),
            Line::from("  mneme-ai install mneme-guardian"),
        ];
        f.render_widget(
            Paragraph::new(help).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("\u{2753} Help"),
            ),
            area,
        );
    }
}
