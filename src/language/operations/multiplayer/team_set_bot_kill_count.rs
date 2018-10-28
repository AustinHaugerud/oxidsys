use language::operations::Operation;

pub struct TeamSetBotKillCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 451;

pub const IDENT: &str = "team_set_bot_kill_count";

impl Operation for TeamSetBotKillCountOp {
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
