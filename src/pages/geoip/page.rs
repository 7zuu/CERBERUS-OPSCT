use std::io::{self, Write};

use crossterm::{cursor, execute, style};

use crate::pages::{home::draw_home_header, settings_typing::SettingsMenu};
use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use viuer::{print_from_file, Config};


pub fn draw_geoip<W: Write>(stdout: &mut W) -> io::Result<()> {
    draw_home_header(stdout)?;
    draw_geoip_header(stdout)?;
    draw_geoip_footer(stdout)?;
    Ok(())
}

pub fn draw_geoip_header<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 08), crossterm::style::Print("# [home/geoip] GEO IP Tracker                                                            #"),
        cursor::MoveTo(0, 09), crossterm::style::Print("#------------------------------------------------------------------------------#"),
        style::SetAttribute(style::Attribute::Reset),
    )
}

pub fn draw_geoip_footer<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 40), crossterm::style::Print("#------------------------------------------------------------------------------#"),
        cursor::MoveTo(0, 41), crossterm::style::Print("#    [q] Quit [h] Home || Check where is the ip originated!                    #"),
        cursor::MoveTo(0, 42), crossterm::style::Print("#------------------------------------------------------------------------------#"),
        style::SetAttribute(style::Attribute::Reset),
    )
}

pub fn draw_maps_display<W: Write>(stdout: &mut W)-> io::Result<()>{
    execute!(stdout)
}