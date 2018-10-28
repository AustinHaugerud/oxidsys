use language::operations::Operation;

pub struct PlayerSetIsMutedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 440;

pub const IDENT: &str = "player_set_is_muted";

impl Operation for PlayerSetIsMutedOp {
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
