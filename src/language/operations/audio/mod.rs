use language::operations::{Operation};
pub mod music_set_culture;
pub mod music_set_situation;
pub mod play_cue_track;
pub mod play_sound;
pub mod play_sound_at_position;
pub mod play_track;
pub mod stop_all_sounds;
pub mod stop_sound_channel;
pub mod store_last_sound_channel;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(music_set_culture::MusicSetCultureOp {}));
    result.push(Box::new(music_set_situation::MusicSetSituationOp {}));
    result.push(Box::new(play_cue_track::PlayCueTrackOp {}));
    result.push(Box::new(play_sound::PlaySoundOp {}));
    result.push(Box::new(play_sound_at_position::PlaySoundAtPositionOp {}));
    result.push(Box::new(play_track::PlayTrackOp {}));
    result.push(Box::new(stop_all_sounds::StopAllSoundsOp {}));
    result.push(Box::new(stop_sound_channel::StopSoundChannelOp {}));
    result.push(Box::new(
        store_last_sound_channel::StoreLastSoundChannelOp {},
    ));
    result
}
