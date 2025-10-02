use std::io::{self, Write};

use crossterm::{cursor, execute};

use crate::pages::settings_typing::SettingsMenu;

pub fn draw_settings<W: Write>(stdout: &mut W, submenu: &SettingsMenu) -> io::Result<()> {
    let submenu_text = match submenu {
        SettingsMenu::General => "General Settings",
        SettingsMenu::Advanced => "Advanced Settings",
    };

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        crossterm::style::Print("Settings Page"),
        cursor::MoveTo(0, 1),
        crossterm::style::Print(
            "Press 'g' for General, 'a' for Advanced, 'h' to go to Home, 'q' to quit."
        ),
        cursor::MoveTo(0, 2),
        crossterm::style::Print(format!("Current submenu: {}", submenu_text))
    )
}
