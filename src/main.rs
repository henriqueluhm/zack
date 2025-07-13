use app::App;
use std::{env, path::PathBuf};

mod app;
mod event;
mod types;
mod ui;

#[cfg(feature = "debug-logs")]
fn init_logging() {
    use simplelog::*;
    let _ = CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        std::fs::File::create("zack.log").unwrap(),
    )]);
}

#[cfg(not(feature = "debug-logs"))]
fn init_logging() {}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_logging();

    let terminal = ratatui::init();

    let maybe_path = env::args().nth(1).map(PathBuf::from);

    let file_content = maybe_path
        .as_ref()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .unwrap_or_else(|| String::from(""));

    let result = App::new(file_content, maybe_path).run(terminal);

    ratatui::restore();

    result
}
