use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use crate::metadata::Metadata;

use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Player {
    pub shuffle: bool,
    pub rate: f64,
    pub volume: f64,
    pub loop_status: String,

    playing: bool,
    queue: VecDeque<Metadata>,
    index: HashMap<String, usize>,
    playing_from_timestamp: u64,
    playing_from_instant: Instant,
    current_track: isize,
    total_tracks: usize,
}

fn create_index(queue: &[Metadata]) -> HashMap<String, usize> {
    let mut index = HashMap::new();
    for (i, m) in queue.iter().enumerate() {
        index.insert(m.clone().trackid, i);
    }
    return index;
}

// TODO: shuffling
impl Player {
    pub fn new(queue: &[Metadata]) -> Player {
        let p = Player {
            shuffle: false,
            rate: 1.0,
            volume: 1.0,
            loop_status: "None".to_string(),
            playing: true,
            queue: VecDeque::from_iter(queue.iter().cloned()),
            index: create_index(queue),
            playing_from_timestamp: 0,
            playing_from_instant: Instant::now(),
            current_track: 0,
            total_tracks: queue.len(),
        };
        // since we are only simulating a music player and aren't Actually a music player, it should be
        // fine to only check if we have to go to the next track every 50 millis
        let mut play = p.clone();
        let tick = chan::tick(Duration::from_millis(50));
        thread::spawn(move || loop {
            chan_select! {
                tick.recv() => {
                    if play.get_position() >= play.current_metadata().length {
                        play.next()
                    }
                },
            }
        });

        return p;
    }
    pub fn playback_status(&self) -> String {
        if self.playing {
            return "Playing".to_string();
        }
        return "Paused".to_string();
    }

    pub fn current_metadata(&self) -> Metadata {
        self.queue[self.current_track as usize].to_owned()
    }

    pub fn get_playing(&self) -> bool {
        self.playing
    }

    pub fn set_playing(&mut self, playing: bool) {
        if self.playing && !playing {
            self.playing_from_timestamp =
                self.playing_from_timestamp + self.playing_from_instant.elapsed().as_millis() as u64
        } else if !self.playing && playing {
            self.playing_from_instant = Instant::now();
        }
        self.playing = playing;
    }

    #[inline(always)]
    pub fn play_pause(&mut self) {
        self.set_playing(!self.get_playing())
    }

    pub fn get_position(&self) -> u64 {
        if !self.playing {
            return self.playing_from_timestamp;
        }
        self.playing_from_timestamp + self.playing_from_instant.elapsed().as_millis() as u64
    }

    pub fn set_position(&mut self, position: u64) {
        if position >= self.current_metadata().length {
            self.next();
            return;
        }
        self.playing_from_timestamp = position;
        self.playing_from_instant = Instant::now();
    }

    pub fn seek(&mut self, offset: i64) {
        // TODO: check for negative position?
        self.playing_from_timestamp = ((self.playing_from_timestamp as i64) + offset) as u64;
        self.playing_from_instant = Instant::now();
    }

    pub fn next(&mut self) {
        self.set_track(self.current_track + 1);
        self.set_position(0);
    }

    pub fn previous(&mut self) {
        self.set_track(self.current_track - 1);
        self.set_position(0);
    }

    // TODO: use list operations instead (pop/push)
    pub fn set_track(&mut self, track: isize) {
        if track >= self.total_tracks as isize {
            self.current_track = 0
        } else if track <= 0 {
            self.current_track = self.total_tracks as isize - 1
        } else {
            self.current_track = track
        }
    }
}
