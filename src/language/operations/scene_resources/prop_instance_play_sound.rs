use language::operations::Operation;

pub struct PropInstancePlaySoundOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1881;

pub const IDENT: &str = "prop_instance_play_sound";

impl Operation for PropInstancePlaySoundOp {
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
