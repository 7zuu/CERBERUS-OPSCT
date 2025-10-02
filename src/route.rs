use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEvent}, execute, terminal::{size, Clear, ClearType, SetSize}
};
use std::{io::{self, Write}, time::Duration};

use crate::pages::{
    geoip::{handle_geoip_events, page::draw_geoip},
    home::draw_home,
    home_events::handle_home_events,
    settings::draw_settings,
    settings_events::handle_settings_events,
    settings_typing::SettingsMenu,
};

pub enum Page {
    Home,
    Settings(SettingsMenu),
    GeoIP,
}

pub enum Transition {
    To(Page),
    Quit,
    Refresh,
}

fn check_terminal_size() -> io::Result<bool> {
    let (width, height) = size()?;
    Ok(width >= 80 && height >= 43)
}


pub fn run_app<W: Write>(stdout: &mut W) -> io::Result<()> {
    let mut current_page = Page::Home;

    loop {
        // Clear the screen
        execute!(stdout, Clear(ClearType::All))?;

        // Handle events
        match current_page {
            Page::Home => match handle_home_events(stdout)? {
                Transition::To(page) => current_page = page,
                Transition::Quit => return Ok(()),
                Transition::Refresh => draw_home(stdout)?,
            },
            Page::Settings(ref mut submenu) => match handle_settings_events(stdout, submenu)? {
                Transition::To(page) => current_page = page,
                Transition::Quit => return Ok(()),
                Transition::Refresh => draw_settings(stdout, submenu)?,
            },
            Page::GeoIP => match handle_geoip_events(stdout)? {
                Transition::To(page) => current_page = page,
                Transition::Quit => return Ok(()),
                Transition::Refresh => draw_geoip(stdout)?,
            },
        }

    }
}
