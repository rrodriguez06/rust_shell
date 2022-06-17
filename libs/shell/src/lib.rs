use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::poll;
use crossterm::Result;

use lib_app::*;
use lib_ui::*;

use futures_lite::{io::BufReader, prelude::*};

use tui::backend::Backend;
use tui::Terminal;

use std::time::Duration;
use std::io::Read;
// use std::process::{Command, Stdio, Child};
use async_process::{Child, Command, Stdio};

fn cd_completion(app: &mut App) {
    let mut splitted = app.input.trim().split_whitespace().peekable();
    splitted.next();

    let mut arg = String::new();
    if splitted.peek().is_some() {
        arg = splitted.next().unwrap().to_string();
    }

    let mut cmd_path = String::new();
    cmd_path.push_str(&app.path);
    cmd_path.push('/');
    cmd_path.push_str(&arg);

    if std::fs::metadata(&cmd_path).is_ok() {
        let dirs = std::fs::read_dir(&cmd_path).unwrap();
        for dir in dirs {
            let dir = dir.unwrap();
            let path = dir.path();
            let md = std::fs::metadata(&path).unwrap();
            if md.is_dir() == true {
                let name = path.file_name().unwrap().to_str().unwrap();
                if !name.starts_with(".") {
                    if path.starts_with(&cmd_path) {
                        let cmd = "cd ".to_string() + &arg + name + "/";
                        app.completion.push(cmd.to_string());
                        app.completion_display.push(name.to_string());
                    }
                }
            }
        }
    }
}

fn create_completion(app: &mut App) {
    let mut splitted = app.input.trim().split_whitespace();
    let cmd = splitted.next().unwrap();

    match cmd {
        "cd" => {
            cd_completion(app);
        }
        _ => {}
    }
}

pub fn run_command(command: String, app: &mut App) -> Child {
    let mut splitted = command.trim().split_whitespace();
    let cmd = splitted.next().unwrap();

    match cmd {
        "cd" => {
            let args = splitted;
            let path = args.peekable().peek().map_or("/Users/", |x| *x);
            std::env::set_current_dir(path).unwrap();
            app.path = std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let child: Child = match Command::new(cmd).stdout(Stdio::piped()).spawn()
            {
                Ok(child) => child,
                Err(_) => {
                    panic!("failed to execute process");
                }
            };

            return child;
            // return String::from(format!("<h2>cd to: {}</h2>", path));
        }
        // "ls" => {
        //     let mut cmd = Command::new(cmd);
        //     cmd.args(args);

        //     let out = cmd.output().unwrap();
        //     let output = String::from_utf8(out.stdout).unwrap();

        //     let mut result = String::new();
        //     let splitted: Vec<String> = output.split("\n").map(|x| x.to_string()).collect();
        //     for split in splitted {
        //         if split.len() > 0 {
        //             let mut dir = std::env::current_dir()
        //                 .unwrap()
        //                 .to_str()
        //                 .unwrap()
        //                 .to_string();
        //             dir.push('/');
        //             dir.push_str(&split);

        //             if std::fs::metadata(&dir).is_ok() {
        //                 let md = std::fs::metadata(&dir).unwrap();
        //                 if md.is_dir() == true {
        //                     result.push_str("<h2> ");
        //                     result.push_str(&split);
        //                     result.push_str("</h2>");
        //                     result.push('\n');
        //                 } else {
        //                     result.push_str("<i> ");
        //                     result.push_str(&split);
        //                     result.push_str("</i>");
        //                     result.push('\n');
        //                 }
        //             } else {
        //                 return output;
        //             }
        //         }
        //     }
        //     return result;
        // }
        _ => {
            let args = splitted;
            let child: Child = match Command::new(cmd).args(args).stdout(Stdio::piped()).spawn()
            {
                Ok(child) => child,
                Err(_) => {
                    panic!("failed to execute process");
                }
            };

            return child;
        }
    }
}

async fn read_output<B: Backend>(child: &mut Child, app: &mut App, terminal: &mut Terminal<B>) {
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        app.output.push_str(&line.unwrap());
        app.output.push('\n');
        terminal.draw(|f| ui(f, &app));
    }
}

pub async fn events<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<bool> {
    if poll(Duration::from_millis(100))? {
        if let Ok(Event::Key(key)) = event::read() {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let command: String = app.input.drain(..).collect();
                        app.history.push(command.clone());
                        app.output.clear();
                        if command == "c" {
                            app.output = "".to_string();
                        } else if command == "help" {
                            app.input_mode = InputMode::Helper;
                        } else {
                            // app.output = run_command(command, app);
                            app.command = command;
                            app.input_mode = InputMode::Output;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Tab => {
                        app.completion.clear();
                        app.completion_display.clear();
                        app.completion_index = 0;
                        create_completion(app);
                        app.input_mode = InputMode::Completion;
                    }
                    KeyCode::Down => {
                        app.history_index = 0;
                        app.input_mode = InputMode::History;
                    }
                    _ => {}
                },
                InputMode::Completion => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Editing;
                        let comp: String = app.completion[app.completion_index].clone();
                        app.input = comp;
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Tab => {
                        app.completion_index += 1;
                        if app.completion.is_empty() == true {
                            app.completion_index = 0;
                        } else if app.completion_index > app.completion.len() - 1 {
                            app.completion_index = 0;
                        }
                    }
                    _ => {}
                },
                InputMode::History => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Editing;
                        let hist: String = app.history[app.history_index].clone();
                        app.input = hist;
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Tab => {
                        app.history_index += 1;
                        if app.history.is_empty() == true {
                            app.history_index = 0;
                        } else if app.history_index > app.history.len() - 1 {
                            app.history_index = 0;
                        }
                    }
                    _ => {}
                },
                InputMode::Helper => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Editing;
                    }
                    _ => {}
                },
                InputMode::Output => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Editing;
                    }
                    _ => {}
                },
            }
            terminal.draw(|f| ui(f, &app))?;
        }
    } else {
        match app.input_mode {
            InputMode::Output => {
                let command = &app.command;
                let mut child = run_command(command.to_string(), app);
                read_output(&mut child, app, terminal).await;
                app.input_mode = InputMode::Editing;
                terminal.draw(|f| ui(f, &app))?;
            }
            _ => {}
        }
    }

    return Ok(true);
}
