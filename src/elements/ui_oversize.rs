use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent},
    execute, style,
    terminal::{disable_raw_mode, size, Clear, ClearType, SetSize},
};

use crate::route::Transition;

pub fn detect<W: Write>(stdout: &mut W) -> io::Result<Option<Transition>> {
    execute!(stdout, SetSize(80, 43))?;
    // loop {
    //     execute!(stdout, Clear(ClearType::All))?;

    //     let (width, height) = size()?;
    //     let is_on = width >= 80 && height >= 43;
    //     let sparator_dynamic = "-".repeat((width - 2) as usize);
    //     let wording = "Size too small, resize to 80(w) x 43(h)";
    //     let wordlen = wording.len() as u16;
    //     if !is_on {
    //         execute!(stdout, Clear(ClearType::All))?;
    //         let warn_location = (width / 2) - (wordlen / 2);
    //         execute!(
    //             stdout,
    //             cursor::MoveTo(0, 0),
    //             crossterm::style::Print(format!("#{}#", sparator_dynamic)),
    //             cursor::MoveTo(warn_location, 2),
    //             crossterm::style::Print(wording),
    //             cursor::MoveTo(6, height - 7),
    //             crossterm::style::Print(format!(
    //                 "{}<-{}/80->",
    //                 " ".repeat(((width / 2) - 10) as usize),
    //                 width
    //             )),
    //             cursor::MoveTo(0, height - 8),
    //             crossterm::style::Print("-----"),
    //             cursor::MoveTo(6, height - 8),
    //             crossterm::style::Print(format!("{}", "-".repeat((width - 6) as usize))),
    //         )?;
    //         for i in 1..height - 1 {
    //             if i == height - 8 {
    //                 execute!(stdout, cursor::MoveTo(5, i), crossterm::style::Print("+"),)?;
    //                 continue;
    //             }
    //             execute!(stdout, cursor::MoveTo(5, i), crossterm::style::Print("|"),)?;
    //         }
    //         execute!(
    //             stdout,
    //             cursor::MoveTo(0, height - 3),
    //             crossterm::style::Print(format!("#{}#", sparator_dynamic)),
    //             cursor::MoveTo(0, height - 2),
    //             crossterm::style::Print(format!(
    //                 "#    Press [esc] to Quit{}#",
    //                 " ".repeat((width - 25) as usize)
    //             )),
    //             cursor::MoveTo(0, height - 1),
    //             crossterm::style::Print(format!("#{}#", sparator_dynamic)),
    //             style::SetAttribute(style::Attribute::Reset),
    //         )?;
    //         if event::poll(Duration::from_millis(100))? {
    //             if let Event::Key(KeyEvent {
    //                 code: KeyCode::Esc, ..
    //             }) = event::read()?
    //             {
    //                 disable_raw_mode()?;
    //                 execute!(stdout, DisableMouseCapture)?;
    //                 return Ok(Some(Transition::Quit));
    //             }
    //         }
    //         continue;
    //     }

    //     execute!(stdout, Clear(ClearType::All))?;
    //     break;
    // }
    Ok(None)
}
