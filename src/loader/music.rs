use std::fs;
use component::music::MusicTrack;

pub fn load_music(music_dir : &str) -> Result<Vec<MusicTrack>, String> {
    let paths =
        fs::read_dir(music_dir).map_err(|_| "Failed to access music directory.".to_owned())?;

    let mut definitions = vec![];

    for path in paths {
        let entry = path.map_err(|_| "Failed reading entry from music directory.")?;

        if entry.path().is_file() {
            let music_track = MusicTrack::from_file(&entry.path())?;
            definitions.push(music_track);
        }
    }

    Ok(definitions)
}
