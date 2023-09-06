use clap::Parser;
use core::{chip8::Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};

use std::{
    error::Error,
    fs::{self},
    io::{self},
    path::Path,
    time::{Duration, Instant},
};

#[cfg(feature = "profile")]
use std::thread;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ROM file to load
    rom: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Loading ROM: {}", &cli.rom);
    let rom = fs::read(Path::new(&cli.rom))?;
    let mut chip8 = Chip8::new();
    chip8.load(&*rom);

    #[cfg(feature = "profile")]
    loop {
        thread::sleep(Duration::from_millis(((1.0 / 10 as f64) * 50.0) as u64));
        chip8.emulate_cycle();
        chip8.tick_timers();
        for (idx, on) in chip8.get_display().screen.iter().enumerate() {
            if *on {
                print!("X");
            } else {
                print!(".");
            }
            if idx % SCREEN_WIDTH == 0 {
                println!();
            }
        }
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create emulator
    let tick_rate = Duration::from_millis(1);

    // run
    let res = run_emulator(&mut terminal, chip8, tick_rate);

    // restore terminal
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

fn run_emulator<B: Backend>(
    terminal: &mut Terminal<B>,
    mut chip8: Chip8,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        chip8.clean_keyboard();
        
        let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let Some(code) = key2code(key.code) {
                    chip8.keypress(code, false);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(());
                }
            };
        };

        chip8.emulate_cycle();
        terminal.draw(|f| ui(f, &chip8, 1))?;
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            chip8.tick_timers();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, chip8: &Chip8, scale: usize) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Chip8"))
        .marker(Marker::Block)
        .paint(|ctx| {
            let display = chip8.get_display();
            for (idx, on) in display.screen.iter().enumerate() {
                let (x, y) = display.to_xy(idx);

                ctx.draw(&Rectangle {
                    x: (x * scale) as f64,
                    y: (SCREEN_HEIGHT - y * scale) as f64,
                    width: (1 * scale) as f64,
                    height: (1 * scale) as f64,
                    color: if *on { Color::White } else { Color::Black },
                });
            }
        })
        .x_bounds([0.0, (SCREEN_WIDTH * scale) as f64])
        .y_bounds([0.0, (SCREEN_HEIGHT * scale) as f64]);
    f.render_widget(canvas, f.size());
}

/*
    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+
*/
fn key2code(keycode: KeyCode) -> Option<u8> {
    match keycode {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('q') => Some(0x4),
        KeyCode::Char('w') => Some(0x5),
        KeyCode::Char('e') => Some(0x6),
        KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('a') => Some(0x7),
        KeyCode::Char('s') => Some(0x8),
        KeyCode::Char('d') => Some(0x9),
        KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('x') => Some(0x0),
        KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('v') => Some(0xF),
        _ => None
    }
}
