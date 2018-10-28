use language::operations::Operation;

pub struct PlayerSetKillCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 434;

pub const IDENT: &str = "player_set_kill_count";

impl Operation for PlayerSetKillCountOp {
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
