mod app;
mod event;
mod ui;

use app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = App::new().run(terminal);

    ratatui::restore();

    result
}
