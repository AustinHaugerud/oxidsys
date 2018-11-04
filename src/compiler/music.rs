use compiler::Compile;
use loader::music::load_music;
use std::fs;

pub struct MusicCompiler {
    music_dir : String,
}

impl MusicCompiler {
    pub fn new(dir : &str) -> MusicCompiler {
        MusicCompiler {
            music_dir : dir.to_owned()
        }
    }
}

impl Compile for MusicCompiler {
    fn compile(&self) -> Result<(), String> {

        let music_definitions = load_music(&self.music_dir)?;

        let mut content = format!("{}\n", music_definitions.len());

        for track in music_definitions {
            content += &format!("{} {} {}\n", track.get_file_path(), track.get_flags(), track.get_flags() | track.get_continue_flags());
        }

        fs::write("music.txt", content).map_err(|_| "Failed to write to music.txt".to_owned())?;

        Ok(())
    }
}
