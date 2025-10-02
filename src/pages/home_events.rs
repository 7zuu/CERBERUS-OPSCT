use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind,
    },
    execute, style,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::{elements::ui_oversize, route::{Page, Transition}};

use super::{home::{draw_home_footer, draw_home_header, HOME_MENU}, settings_typing::SettingsMenu};

const VISIBLE_ROWS: usize = 13;

pub fn handle_home_events<W: Write>(stdout: &mut W) -> io::Result<Transition> {
    enable_raw_mode()?;
    execute!(stdout, EnableMouseCapture)?;
    let mut start_index = 0;
    let mut active_index = 0;
    let offset:usize = 12;

    loop {
        execute!(stdout, Clear(ClearType::All))?;
        match ui_oversize::detect(stdout) {
            Ok(Some(Transition::Quit)) => {
                disable_raw_mode()?;
                execute!(stdout, DisableMouseCapture)?;
                return Ok(Transition::Quit);
            }
            Ok(None) => {}
            Err(e) => return Err(e),
            _=>{}
        };

        draw_home_header(stdout)?;
        for i in 0..VISIBLE_ROWS {
            if let Some(line) = HOME_MENU.get(start_index + i) {
                execute!(stdout, cursor::MoveTo(0, (i as u16) + (offset as u16)))?;
                if i == active_index {
                    execute!(
                        stdout,
                        style::SetAttribute(style::Attribute::Bold),
                        style::SetForegroundColor(style::Color::Cyan),
                        style::Print("> "),
                        style::Print(line),
                        style::Print("  "),
                        style::SetAttribute(style::Attribute::Reset),
                        style::SetForegroundColor(style::Color::White),
                    )?;
                } else {
                    writeln!(stdout, "  {}", line)?;
                }
            }
        }
        draw_home_footer(stdout)?;

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(stdout, DisableMouseCapture)?;
                    return Ok(Transition::Quit);
                }
                KeyCode::Char('s') => {
                    disable_raw_mode()?;
                    execute!(stdout, DisableMouseCapture)?;
                    return Ok(Transition::To(Page::Settings(SettingsMenu::General)));
                }
                KeyCode::Up => {
                    if active_index > 0 {
                        active_index -= 1;
                    } else if start_index > 0 {
                        start_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if active_index < VISIBLE_ROWS - 1 && active_index < HOME_MENU.len() - 1 {
                        active_index += 1;
                    } else if start_index + VISIBLE_ROWS < HOME_MENU.len() {
                        start_index += 1;
                    }
                }
                KeyCode::Enter => {
                    disable_raw_mode()?;
                    execute!(stdout, DisableMouseCapture)?;
                    return Ok(match HOME_MENU[active_index] {
                        "Web Scanner" => Transition::To(Page::Home),
                        "Framework Scanner" => Transition::To(Page::Home),
                        "Geo IP" => Transition::To(Page::GeoIP),
                        "Network Tools" => Transition::To(Page::Home),
                        _ => Transition::To(Page::Home),
                    });
                }
                _ => {}
            },
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    // Calculate the row index based on the mouse click position
                    let row = mouse_event.row as usize;
                    if row >= offset && row < offset + VISIBLE_ROWS {
                        let relative_row = row - offset;
                        if relative_row < HOME_MENU.len() {
                            active_index = relative_row;
                        }
                    }
                }
                MouseEventKind::ScrollUp => {
                    if active_index > 0 {
                        active_index -= 1;
                    } else if start_index > 0 {
                        start_index -= 1;
                    }
                }
                MouseEventKind::ScrollDown => {
                    if active_index < VISIBLE_ROWS - 1 && active_index < HOME_MENU.len() - 1 {
                        active_index += 1;
                    } else if start_index + VISIBLE_ROWS < HOME_MENU.len() {
                        start_index += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
