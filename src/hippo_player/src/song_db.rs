
use std::collections::HashMap;

use std::num::ParseIntError;

// TODO: Generated from C-API?
const HIPPOMETADATAKEY_TITLE: usize = 0;
const HIPPOMETADATAKEY_TYPE: usize = 1;
const HIPPOMETADATAKEY_DURATION: usize = 2;
const HIPPOMETADATAKEY_AUTHORINGTOOL: usize = 3;
const HIPPOMETADATAKEY_ARTIST: usize = 4;
const HIPPOMETADATAKEY_DATE: usize = 5;
const HIPPOMETADATAKEY_MESSAGE: usize = 6;
const HIPPOMETADATAKEY_CHANNELSCOUNT: usize = 8;
const HIPPOMETADATAKEY_PATTERNSCOUNT: usize = 9;
const HIPPOMETADATAKEY_PATTERNNAMES: usize = 10;
const HIPPOMETADATAKEY_SAMPLECOUNT: usize = 11;
const HIPPOMETADATAKEY_SAMPLES: usize = 12;

#[derive(Default, Debug)]
struct SongInfo {
    title: String,
    song_type: String,
    duration: f32,
    authoring_tool: String,
    artist: String,
    date: String,
    message: String,
    channels_count: usize,
    patterns_count: usize,
    pattern_names: Vec<String>,
    samples: Vec<String>,
}

#[derive(Default, Debug)]
struct SongEntry {
    song: SongInfo,
    sub_songs: HashMap<usize, SongInfo>,
}

pub struct SongDb {
    songs: HashMap<String, SongEntry>,
}


impl SongDb {
    fn update_song_info(entry: &mut SongInfo, key: usize, value: &str) -> Result<usize, ParseIntError>  {
        match key {
            HIPPOMETADATAKEY_TITLE => entry.title = value.to_owned(),
            HIPPOMETADATAKEY_TYPE => entry.song_type = value.to_owned(),
            HIPPOMETADATAKEY_DURATION => entry.duration = value.parse::<f32>().unwrap(),    // TODO: Fix me
            HIPPOMETADATAKEY_AUTHORINGTOOL => entry.authoring_tool = value.to_owned(),
            HIPPOMETADATAKEY_ARTIST => entry.artist = value.to_owned(),
            HIPPOMETADATAKEY_DATE => entry.date = value.to_owned(),
            HIPPOMETADATAKEY_MESSAGE => entry.message = value.to_owned(),
            HIPPOMETADATAKEY_CHANNELSCOUNT => entry.channels_count = value.parse::<usize>()?,
            _ => (),
        }

        Ok(0)
    }

    fn update_entry(entry: &mut SongEntry, key: usize, sub_song: usize, value: &str) {
        if sub_song == 0 {
            Self::update_song_info(&mut entry.song, key, value);
        } else {
            // Fix: NLL
            let mut updated_entry = false;

            if let Some(song_info) = entry.sub_songs.get_mut(&sub_song) {
                updated_entry = true;
                Self::update_song_info(song_info, key, value);
            }

            if !updated_entry {
                let mut song_info = SongInfo::default();
                Self::update_song_info(&mut song_info, key, value);
                entry.sub_songs.insert(sub_song, song_info);
            }
        }
    }

    pub fn set_key(&mut self, resource: &str, sub_song: usize, key: usize, value: &str) {
        // Fix: NLL
        let mut updated_entry = false;

        if let Some(entry) = self.songs.get_mut(resource) {
            updated_entry = true;
            Self::update_song_info(&mut entry.song, key, value);
        }

        if !updated_entry {
            let mut entry = SongEntry::default();
            Self::update_song_info(&mut entry.song, key, value);
            self.songs.insert(resource.to_owned(), entry);
        }
    }
}
