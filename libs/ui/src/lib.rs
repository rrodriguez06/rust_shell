use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use unicode_width::UnicodeWidthStr;

use lib_app::*;

fn display_completion<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let mut completion_string: Vec<Span> = Vec::new();
    let mut index = 0;
    let mut i = 0;
    for comp in app.completion_display.iter() {
        if i < app.completion_index {
            index += comp.len() + 1;
        }
        if i == app.completion_index {
            completion_string.push(Span::styled(comp, Style::default().fg(Color::Blue)));
        } else {
            completion_string.push(Span::raw(comp));
        }
        completion_string.push(Span::raw(" "));
        i += 1;
    }
    let completion = Paragraph::new(Text::from(Spans::from(completion_string)))
        .block(Block::default().borders(Borders::ALL).title("Completion"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(completion, chunk);

    f.set_cursor(chunk.x + index as u16 + 1, chunk.y + 1);
}

fn display_history<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let mut history_string: Vec<Span> = Vec::new();
    let mut index = 0;
    let mut i = 0;
    for hist in app.history.iter() {
        if i < app.history_index {
            index += hist.len() + 3;
        }
        if i == app.history_index {
            history_string.push(Span::styled("\"", Style::default().fg(Color::Blue)));
            history_string.push(Span::styled(hist, Style::default().fg(Color::Blue)));
            history_string.push(Span::styled("\"", Style::default().fg(Color::Blue)));
        } else {
            history_string.push(Span::raw("\""));
            history_string.push(Span::raw(hist));
            history_string.push(Span::raw("\""));
        }
        history_string.push(Span::raw(" "));
        i += 1;
    }
    let history = Paragraph::new(Text::from(Spans::from(history_string)))
        .block(Block::default().borders(Borders::ALL).title("History"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(history, chunk);

    f.set_cursor(chunk.x + index as u16 + 1, chunk.y + 1);
}

fn display_output<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let lines: Vec<String> = app.output.lines().map(|x| String::from(x)).collect();

    let text = construct_message(&lines);

    let output = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Outputs"))
        .style(match app.input_mode {
            InputMode::Output => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

    f.render_widget(output, chunk);
}

fn create_popup<B: Backend>(
    f: &mut Frame<B>,
    _app: &App,
    text: Vec<Spans>,
    size_x: u16,
    size_y: u16,
    title: &str,
) {
    let popup = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::Yellow));

    let popup_layout_y = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - size_y) / 2),
                Constraint::Percentage(size_y),
                Constraint::Percentage((100 - size_y) / 2),
            ]
            .as_ref(),
        )
        .split(f.size());

    let popup_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - size_x) / 2),
                Constraint::Percentage(size_x),
                Constraint::Percentage((100 - size_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout_y[1])[1];

    f.render_widget(Clear, popup_layout);
    f.render_widget(popup, popup_layout);
}

fn contains_patterns(line: &String, patterns: &Vec<&str>) -> bool {
    for pattern in patterns.iter() {
        if line.contains(pattern) {
            return true;
        }
    }
    return false;
}

fn get_patterned(line: String, start_pattern: &str, end_pattern: &str) -> (String, String, String) {
    if line.contains(start_pattern) {
        let start = line.find(start_pattern).unwrap();
        let end = line.find(end_pattern).unwrap();

        let before = String::from(&line[0..start]);
        let between = String::from(&line[(start + start_pattern.len())..end]);
        let after = String::from(&line[(end + end_pattern.len())..]);
        return (before, between, after);
    }

    return (String::from(""), String::from(""), String::from(line));
}

fn construct_line(line: String, text: &mut Vec<Span>) {
    let mut rest = String::from(line);

    let start_patterns = vec!["<h1>", "<h2>", "<c>", "<i>"];
    let end_patterns = vec!["</h1>", "</h2>", "</c>", "</i>"];

    while contains_patterns(&rest, &start_patterns) {
        for i in 0..start_patterns.len() {
            let (before, between, after) = get_patterned(rest, start_patterns[i], end_patterns[i]);
            if !before.is_empty() {
                construct_line(before, text);
            }
            if !between.is_empty() {
                match i {
                    0 => {
                        let style = Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::UNDERLINED);

                        text.push(Span::styled(between, style));
                    }
                    1 => {
                        let style = Style::default().fg(Color::Blue);

                        text.push(Span::styled(between, style));
                    }
                    2 => {
                        let style = Style::default().fg(Color::Red);

                        text.push(Span::styled(between, style));
                    }
                    3 => {
                        let style = Style::default().fg(Color::Magenta);

                        text.push(Span::styled(between, style));
                    }
                    _ => {}
                }
            }
            rest = after;
        }
    }

    text.push(Span::raw(rest));
}

fn construct_message(lines: &Vec<String>) -> Vec<Spans> {
    let mut texts: Vec<Spans> = Vec::new();

    for line in lines.iter() {
        let mut text: Vec<Span> = Vec::new();

        construct_line(line.to_string(), &mut text);

        texts.push(Spans::from(text));
    }

    texts
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to enter insert mode."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit insert mode, "),
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to enter completion mode, "),
                Span::styled("Down", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to enter history mode, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to launch command."),
            ],
            Style::default(),
        ),
        InputMode::Completion => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit completion mode, "),
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select completion element, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to accept completion."),
            ],
            Style::default(),
        ),
        InputMode::History => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit history mode, "),
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select history element, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to accept history."),
            ],
            Style::default(),
        ),
        InputMode::Helper => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit helper mode."),
            ],
            Style::default(),
        ),
        InputMode::Output => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit output mode."),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::Completion => Style::default(),
            InputMode::History => Style::default(),
            InputMode::Helper => Style::default(),
            InputMode::Output => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title(&*app.path));
    f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1)
        }
        InputMode::Completion => {
            display_completion(f, app, chunks[2]);
        }
        InputMode::History => {
            display_history(f, app, chunks[2]);
        }
        InputMode::Helper => {
            let message = construct_message(&app.helper);
            create_popup(f, app, message, 30, 40, "Helper");
        }
        InputMode::Output => {}
    }

    display_output(f, app, chunks[3]);
}
