use language::operations::Operation;

pub struct PlayCueTrackOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 602;

pub const IDENT: &str = "play_cue_track";

impl Operation for PlayCueTrackOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
