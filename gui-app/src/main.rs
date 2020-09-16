use chip_8_emulator_gui_app::{App, Error};
use std::env;

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let program_path = args.nth(1).unwrap();

    let mut app = App::init()?;
    app.load_program(&program_path)?;
    app.run()?;

    Ok(())
}
