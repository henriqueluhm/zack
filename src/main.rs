mod app;
mod event;
mod types;
mod ui;

use app::App;

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
    let result = App::new().run(terminal);

    ratatui::restore();

    result
}
