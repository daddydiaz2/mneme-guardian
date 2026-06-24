/// TUI for mneme-guardian — dashboard, config, providers, review logs.
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs},
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
    Providers,
    Logs,
    Help,
}

const TABS: &[&str] = &[
    "📊 Dashboard",
    "⚙ Config",
    "🔌 Providers",
    "📋 Logs",
    "❓ Help",
];

struct App {
    config: config::Config,
    screen: Screen,
    tab: usize,
    log: Vec<String>,
}

impl App {
    fn new(cfg: config::Config) -> Self {
        Self {
            config: cfg,
            screen: Screen::Dashboard,
            tab: 0,
            log: Vec::new(),
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
                    self.screen = Screen::Providers;
                    self.tab = 2;
                    true
                }
                KeyCode::Char('3') => {
                    self.screen = Screen::Logs;
                    self.tab = 3;
                    true
                }
                KeyCode::Char('4') => {
                    self.screen = Screen::Help;
                    self.tab = 4;
                    true
                }
                KeyCode::Char('q') | KeyCode::Esc => false,
                _ => true,
            },
            Screen::Config => match key {
                KeyCode::Esc | KeyCode::Backspace => {
                    self.screen = Screen::Dashboard;
                    self.tab = 0;
                    true
                }
                KeyCode::Char('q') => false,
                _ => true,
            },
            Screen::Providers => match key {
                KeyCode::Esc | KeyCode::Backspace => {
                    self.screen = Screen::Dashboard;
                    self.tab = 0;
                    true
                }
                KeyCode::Char('q') => false,
                _ => true,
            },
            Screen::Logs => match key {
                KeyCode::Esc | KeyCode::Backspace => {
                    self.screen = Screen::Dashboard;
                    self.tab = 0;
                    true
                }
                KeyCode::Char('q') => false,
                KeyCode::Char('c') => {
                    self.log.clear();
                    true
                }
                _ => true,
            },
            Screen::Help => match key {
                KeyCode::Esc | KeyCode::Backspace => {
                    self.screen = Screen::Dashboard;
                    self.tab = 0;
                    true
                }
                KeyCode::Char('q') => false,
                _ => true,
            },
        }
    }

    fn render(&self, f: &mut Frame) {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
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
            Screen::Providers => self.render_providers(f, chunks[1]),
            Screen::Logs => self.render_logs(f, chunks[1]),
            Screen::Help => self.render_help(f, chunks[1]),
        }
    }

    fn render_dashboard(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mneme_ok = crate::mneme::find_mneme().is_some();
        let status = vec![
            Line::from(format!("Provider: {}", self.config.provider)),
            Line::from(format!(
                "Model: {}",
                self.config.model.as_deref().unwrap_or("default")
            )),
            Line::from(format!(
                "Mneme sync: {}",
                if self.config.mneme_enabled {
                    "✅"
                } else {
                    "❌"
                }
            )),
            Line::from(format!(
                "Exit on issues: {}",
                if self.config.exit_on_issues {
                    "✅"
                } else {
                    "❌"
                }
            )),
            Line::from(format!("Max lines: {}", self.config.max_lines)),
            Line::from(""),
            Line::from(Span::styled(
                if mneme_ok {
                    "mneme-brain: ✅ connected"
                } else {
                    "mneme-brain: ❌ not found"
                },
                Style::default().fg(if mneme_ok { Color::Green } else { Color::Red }),
            )),
        ];
        f.render_widget(
            Paragraph::new(status).block(Block::default().borders(Borders::ALL).title("📊 Status")),
            chunks[0],
        );

        let actions = vec![
            Line::from(Span::styled(
                "1: Config  2: Providers  3: Logs  4: Help",
                Style::default().fg(Color::Cyan),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "CLI commands:",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  mneme-g init     — Create config"),
            Line::from("  mneme-g install  — Install hook"),
            Line::from("  mneme-g run      — Review staged"),
            Line::from("  mneme-g run --ci — CI mode"),
        ];
        f.render_widget(
            Paragraph::new(actions)
                .block(Block::default().borders(Borders::ALL).title("⚡ Actions")),
            chunks[1],
        );
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
            Paragraph::new(config_text)
                .block(Block::default().borders(Borders::ALL).title("⚙ Config")),
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
                Row::new(vec![*name, *desc, if available { "✅" } else { " " }])
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
        .block(Block::default().borders(Borders::ALL).title("🔌 Providers"));
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
                    .title("📋 Review Logs"),
            ),
            area,
        );
    }

    fn render_help(&self, f: &mut Frame, area: Rect) {
        let help = vec![
            Line::from(Span::styled(
                "mneme-guardian — AI Code Review Guardian",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Setup",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  mneme-g init          — Create config"),
            Line::from("  mneme-g install       — Pre-commit hook"),
            Line::from("  mneme-g uninstall     — Remove hook"),
            Line::from(""),
            Line::from(Span::styled(
                "Review",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("  mneme-g run           — Review staged files"),
            Line::from("  mneme-g run --ci      — Review last commit"),
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
            Paragraph::new(help).block(Block::default().borders(Borders::ALL).title("❓ Help")),
            area,
        );
    }
}
