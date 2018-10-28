use language::operations::Operation;

pub struct PlaySoundAtPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 599;

pub const IDENT: &str = "play_sound_at_position";

impl Operation for PlaySoundAtPositionOp {
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
