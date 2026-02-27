mod pages;
mod route;
mod visor;
mod elements;

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use route::run_app;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {    

    enable_raw_mode()?;
    
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide)?;
    
    let res = run_app(&mut stdout);
    
    disable_raw_mode()?;

    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    )?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
