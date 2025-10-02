pub mod toolkit;
pub mod typing;
pub mod page;

use crossterm::{
    cursor::MoveTo, event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
use page::draw_geoip;
use std::io::{self, Write};

use crate::{elements::ui_oversize, route::{Page, Transition}};


pub fn handle_geoip_events<W: Write>(
    stdout: &mut W
) -> io::Result<Transition> {
    enable_raw_mode()?;
    execute!(stdout, EnableMouseCapture)?;

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
        draw_geoip(stdout)?;


        // Move cursor to row 10 (y=10), column 0 (x=0)

        // let mut typing_mode = false;

        // if typing_mode{
            
        //     execute!(stdout, MoveTo(0, 39))?;
        //     write!(stdout, "Type something: ")?;
        //     stdout.flush()?;

        //     let mut typed_input = String::new();
        //     loop {
        //         match event::read()? {
        //             Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
        //                 break;
        //             }
        //             Event::Key(KeyEvent { code: KeyCode::Backspace, .. }) => {
        //                 if !typed_input.is_empty() {
        //                     typed_input.pop();
        //                     execute!(
        //                         stdout,
        //                         MoveTo(0, 10),
        //                         Clear(ClearType::CurrentLine)
        //                     )?;
        //                     write!(stdout, "Type something: {}", typed_input)?;
        //                     stdout.flush()?;
        //                 }
        //             }
        //             Event::Key(KeyEvent { code: KeyCode::Delete, .. }) => {
        //                 if !typed_input.is_empty() {
        //                     typed_input.pop();
        //                     execute!(
        //                         stdout,
        //                         MoveTo(0, 10),
        //                         Clear(ClearType::CurrentLine)
        //                     )?;
        //                     write!(stdout, "Type something: {}", typed_input)?;
        //                     stdout.flush()?;
        //                 }
        //             }
        //             Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => {
        //                 typed_input.push(c);
        //                 write!(stdout, "{}", c)?;
        //                 stdout.flush()?;
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(stdout, DisableMouseCapture)?;
                    return Ok(Transition::Quit)
                },
                KeyCode::Char('h') => {
                    disable_raw_mode()?;
                    execute!(stdout, DisableMouseCapture)?;
                    return Ok(Transition::To(Page::Home))
                },
                _ => {}
            },
            _ => {}
        }
    }
}
