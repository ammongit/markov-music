/*
 * main.rs
 *
 * markov-music - A music player that uses Markov chains to choose songs
 * Copyright (c) 2017 Ammon Smith
 *
 * markov-music is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * markov-music is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with markov-music.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate clap;
extern crate mpv;
extern crate ncurses;
extern crate pancurses;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate toml;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use args::{Args, parse_args};
use config::Config;
use error::Error;
use std::env;
use std::path::Path;
use std::process::exit;
use ui::Command::*;
use ui::{Ui, next_command};
use utils::{HOME_DIR, HOME_DIR_PATH};

#[macro_use]
mod macros {
    macro_rules! curses {
        ($call:expr) => {
            match $call {
                0 => Ok(()),
                _ => Err(::ui::UiError::new(stringify!($call))),
            }
        }
    }

    macro_rules! curses_res {
        ($call:expr) => {
            match $call {
                Ok(x) => Ok(x),
                Err(_) => Err(::ui::UiError::new(stringify!($call))),
            }
        }
    }
}

mod args;
mod config;
mod error;
mod markov;
mod player;
mod song;
mod ui;
mod utils;

fn main() {
    let args = match parse_args() {
        Ok(x) => x,
        Err(e) => {
            println!("Error parsing arguments: {}", e);
            exit(1);
        }
    };

    if let Err(e) = env::set_current_dir(HOME_DIR_PATH.as_path()) {
        println!("Can't switch to home directory '{}': {}", *HOME_DIR, e);
        exit(1);
    }

    if let Err(e) = env::set_current_dir(&args.config.music_dir) {
        println!(
            "Can't switch to music directory '{}': {}",
            args.config.music_dir,
            e
        );
        exit(1);
    }

    let chain: markov::Chain = {
        let path = Path::new(&args.config.storage_file);
        if path.exists() {
            match markov::Chain::read(path) {
                Ok(x) => x,
                Err(e) => {
                    println!("Can't read markov data: {}", e);
                    exit(1);
                }
            }
        } else {
            let chain = markov::Chain::new();
            if let Err(e) = chain.write(path) {
                println!("Can't write markov data: {}", e);
                exit(1);
            }
            chain
        }
    };

    let handle = markov::Handle::new(chain);
    if let Err(e) = main_loop(handle, args) {
        println!("Error: {}", e);
        exit(1);
    }
}

fn main_loop(mut handle: markov::Handle, args: Args) -> Result<(), Error> {
    let mut ui = Ui::new(args.color)?;
    ui.full_redraw(&handle)?;

    loop {
        match next_command(&ui) {
            TogglePause => handle.toggle_pause(),
            Stop => handle.stop()?,
            MoveUp => handle.cursor_up(),
            MoveDown => handle.cursor_down(),
            MoveLeft => handle.cursor_left(),
            MoveRight => handle.cursor_right(),
            PlayCurrent => handle.play()?,
            RaiseVolume => handle.change_volume(args.config.volume_step),
            LowerVolume => handle.change_volume(-args.config.volume_step),
            Mute => handle.toggle_mute(),
            SeekBack => handle.seek(-args.config.seek_seconds)?,
            SeekForward => handle.seek(args.config.seek_seconds)?,
            SeekStart => handle.seek_begin()?,
            SeekEnd => handle.seek_end()?,
            AddSelected => handle.add()?,
            Shuffle => handle.shuffle()?,
            Next => handle.next()?,
            Previous => handle.prev()?,
            Repeat => handle.repeat()?,
            LoopBack => handle.loop_back()?,
            Like => handle.like(),
            Dislike => handle.dislike(),
            Random => handle.random()?,
            Tired => handle.tired(),
            Redraw => ui.full_redraw(&handle)?,
            Quit | Abort => break,
            Nothing => (),
        }
    }

    Ok(())
}
