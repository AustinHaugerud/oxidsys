use language::operations::Operation;

pub struct GameKeyIsDownOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 72;

pub const IDENT: &str = "game_key_is_down";

impl Operation for GameKeyIsDownOp {
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
