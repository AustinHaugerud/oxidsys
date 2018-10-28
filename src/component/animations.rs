pub struct Animation {
    id: String,
    flags: u32,
    master_flags: u32,
    sequences: Vec<AnimationSequence>,
}

pub struct AnimationSequence {
    duration: f32,
    resource: String,
    beg_frame: u32,
    end_frame: u32,
    flags: u32,
}
