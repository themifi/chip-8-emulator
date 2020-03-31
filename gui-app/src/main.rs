use chip_8_emulator_gui_app::App;
use std::env;

fn main() {
    let mut args = env::args();
    let program_path = args.nth(1).unwrap();

    let mut app = App::new();
    app.load_program(&program_path);
    app.run();
}
