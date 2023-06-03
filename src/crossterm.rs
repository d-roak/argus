use crate::{app::App, ui, global_state::State, global_key_handler::handle_key_event};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let state = State::new();

    let app = App::new("web3xplore", enhanced_graphics);
    let res = run_app(&mut terminal, app, state, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    mut state: State,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    crate::screens::blocks::app::update_blocks_list(&mut state);
    let block_number = state.blocks.items[state.blocks.state.selected().unwrap()].clone();
    crate::screens::blocks::app::get_block_by_number(&mut state, &block_number);
    loop {
        terminal.draw(|f| ui::draw(f, &mut state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(&mut state, key);
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if state.should_quit {
            return Ok(());
        }
    }
}
