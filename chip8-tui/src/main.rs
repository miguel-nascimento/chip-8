use clap::Parser;
use core::{chip8::Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};

use std::{
    error::Error,
    fs::{self},
    io::{self},
    path::Path,
    time::{Duration, Instant},
};

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
    // loop {
    //     thread::sleep(Duration::from_millis(((1.0 / 10 as f64) * 100.0) as u64));
    //     chip8.emulate_cycle();
    //     chip8.tick_timers();
    //     for (idx, on) in chip8.get_display().screen.iter().enumerate() {
    //         if *on {
    //             print!("X");
    //         } else {
    //             print!(".");
    //         }
    //         if idx % SCREEN_WIDTH == 0 {
    //             println!();
    //         }
    //     }
    // }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create emulator
    let tick_rate = Duration::from_millis(50);

    let mut chip8 = core::chip8::Chip8::new();
    chip8.load(&*rom);

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
        chip8.emulate_cycle();
        terminal.draw(|f| ui(f, &chip8, 1))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    _ => key2chip(key.code, &mut chip8),
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            chip8.tick_timers();
            chip8.clean_keyboard();
            last_tick = Instant::now();
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

fn key2chip(keycode: KeyCode, chip8: &mut Chip8) {
    match keycode {
        KeyCode::Char('1') => chip8.keypress(0x1, true),
        KeyCode::Char('2') => chip8.keypress(0x2, true),
        KeyCode::Char('3') => chip8.keypress(0x3, true),
        KeyCode::Char('4') => chip8.keypress(0xC, true),
        KeyCode::Char('q') => chip8.keypress(0x4, true),
        KeyCode::Char('w') => chip8.keypress(0x5, true),
        KeyCode::Char('e') => chip8.keypress(0x6, true),
        KeyCode::Char('r') => chip8.keypress(0xD, true),
        KeyCode::Char('a') => chip8.keypress(0x7, true),
        KeyCode::Char('s') => chip8.keypress(0x8, true),
        KeyCode::Char('d') => chip8.keypress(0x9, true),
        KeyCode::Char('f') => chip8.keypress(0xE, true),
        KeyCode::Char('z') => chip8.keypress(0xA, true),
        KeyCode::Char('x') => chip8.keypress(0x0, true),
        KeyCode::Char('c') => chip8.keypress(0xB, true),
        KeyCode::Char('v') => chip8.keypress(0xF, true),
        _ => {}
    }
}
