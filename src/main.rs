mod app;
mod ui;
mod domain;
mod storage;
mod input;

use anyhow::Result;

fn main() -> Result<()> {
    // Initialize app
    let mut app = app::App::new()?;
    
    // Run the application
    app.run()?;
    
    Ok(())
}
