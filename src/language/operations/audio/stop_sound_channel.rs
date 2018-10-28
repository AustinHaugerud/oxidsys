use language::operations::Operation;

pub struct StopSoundChannelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 616;

pub const IDENT: &str = "stop_sound_channel";

impl Operation for StopSoundChannelOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
