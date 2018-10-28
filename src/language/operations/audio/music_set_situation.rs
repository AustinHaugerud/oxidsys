use language::operations::Operation;

pub struct MusicSetSituationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 603;

pub const IDENT: &str = "music_set_situation";

impl Operation for MusicSetSituationOp {
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
