/*
 * handle/object.rs
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

use error::{Error, ErrorCause};
use handle::cursor::Cursor;
use markov::Chain;
use player::{Player, Seek, State};
use std::cmp;
use song::Tags;
use std::path::{Path, PathBuf};

pub struct Handle {
    chain: Chain,
    player: Player,
    cursor: Cursor,
    stopped: bool,
}

impl Handle {
    pub fn new(chain: Chain) -> Self {
        Handle {
            chain: chain,
            player: Player::new(),
            cursor: Cursor::new(),
            stopped: true,
        }
    }

    // Navigator
    pub fn get_current_dir(&self) -> &Path {
        self.cursor.get_path()
    }

    pub fn set_current_dir(&mut self, path: &Path) -> Result<(), Error> {
        if path.is_dir() {
            self.cursor.set_path(PathBuf::from(path))?;
            Ok(())
        } else {
            let message = format!(
                "Not a directory: {}",
                path.to_str().unwrap_or("<invalid UTF-8>")
            );
            Err(Error::new(message, ErrorCause::NoCause()))
        }
    }

    pub fn cursor_up(&mut self) {
        self.cursor.up();
    }

    pub fn cursor_down(&mut self) {
        self.cursor.down();
    }

    pub fn cursor_left(&mut self) {
        self.cursor.left();
    }

    pub fn cursor_right(&mut self) {
        self.cursor.right();
    }

    // Player
    pub fn song_tags(&self) -> Result<Tags, Error> {
        unimplemented!();
    }

    pub fn play_state(&self) -> State {
        if self.stopped {
            State::Stopped
        } else if self.player.is_paused() {
            State::Paused
        } else {
            State::Playing
        }
    }

    pub fn play_percent(&self) -> i32 {
        self.player.percent_pos()
    }

    pub fn toggle_pause(&mut self) {
        let pause = self.player.is_paused();
        self.player.set_pause(!pause);
    }

    pub fn play(&mut self) -> Result<(), Error> {
        let song = self.cursor.current()?;
        self.player.play(song)?;
        self.stopped = false;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.player.stop()?;
        self.stopped = true;

        Ok(())
    }

    pub fn is_muted(&self) -> bool {
        self.player.is_muted()
    }

    pub fn toggle_mute(&mut self) {
        let mute = self.player.is_muted();
        self.player.set_mute(!mute);
    }

    pub fn get_volume(&self) -> i32 {
        self.player.get_volume()
    }

    pub fn change_volume(&mut self, offset: i32) {
        let volume = self.player.get_volume() + offset;
        let volume = cmp::max(cmp::min(volume, 100), 0);
        self.player.set_volume(volume);
    }

    pub fn seek_begin(&mut self) -> Result<(), Error> {
        self.player.seek(Seek::Absolute(0.0))?;

        Ok(())
    }

    pub fn seek_end(&mut self) -> Result<(), Error> {
        self.player.seek(Seek::Absolute(-0.001))?;

        Ok(())
    }

    pub fn seek(&mut self, secs: f32) -> Result<(), Error> {
        assert!(secs.is_finite());
        self.player.seek(Seek::Relative(secs))?;

        Ok(())
    }

    // Markov chain
    pub fn add(&mut self) -> Result<(), Error> {
        let song = self.cursor.current()?;
        unimplemented!();
    }

    pub fn shuffle(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn next(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn prev(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn repeat(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn loop_back(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn like(&mut self) {
        unimplemented!();
    }

    pub fn dislike(&mut self) {
        unimplemented!();
    }

    pub fn tired(&mut self) {
        unimplemented!();
    }

    pub fn random(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}