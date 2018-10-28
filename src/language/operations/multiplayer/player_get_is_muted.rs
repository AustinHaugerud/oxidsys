use language::operations::Operation;

pub struct PlayerGetIsMutedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 439;

pub const IDENT: &str = "player_get_is_muted";

impl Operation for PlayerGetIsMutedOp {
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
