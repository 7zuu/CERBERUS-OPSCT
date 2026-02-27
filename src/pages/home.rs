use std::io::{self, Write};

use crossterm::{cursor, execute, style};

pub fn draw_home<W: Write>(stdout: &mut W) -> io::Result<()> {
    draw_home_header(stdout)?;
    draw_home_footer(stdout)?;
    Ok(())
}

pub fn draw_home_header<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 1) ,crossterm::style::Print("       .  :  :  :   :  :  :: .:   @@   ::   @@   .:  :  :  .   :  .  :   .     "),
        cursor::MoveTo(0, 2) ,crossterm::style::Print("                                 @@@@      @@@@                                "),
        cursor::MoveTo(0, 3) ,crossterm::style::Print("                                @@@ @@    @@@@@@                               "),
        cursor::MoveTo(0, 4) ,crossterm::style::Print("                                @@@ @@@@@@@@ @@@                               "),
        cursor::MoveTo(0, 5) ,crossterm::style::Print("     :  .   :  :  .      .  .   @@  CERBERUS  @@   .  :      .  .  .   .  .    "),
        cursor::MoveTo(0, 6) ,crossterm::style::Print("       .  :  :  :.  :  :  :: .: @@@@@@@@@@@@@@@@  :  :  :  .   .  :  :   .     "),
        style::SetForegroundColor(style::Color::White),
        cursor::MoveTo(0, 7) ,crossterm::style::Print("#------------------------------------------------------------------------------#"),
    )
}

pub fn draw_home_footer<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 40), crossterm::style::Print("#------------------------------------------------------------------------------#"),
        cursor::MoveTo(0, 41), crossterm::style::Print("#    [q] Quit [s] Settings || use your mouse, or arrow to select the tools     #"),
        cursor::MoveTo(0, 42), crossterm::style::Print("#------------------------------------------------------------------------------#"),
        style::SetAttribute(style::Attribute::Reset),
    )
}

pub const HOME_MENU : [&str; 4] = [
    "Web Scanner",
    "Framework Scanner",
    "Geo IP",
    "Network Tools"
];