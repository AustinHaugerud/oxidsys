use language::operations::Operation;

pub struct MusicSetCultureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 604;

pub const IDENT: &str = "music_set_culture";

impl Operation for MusicSetCultureOp {
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
