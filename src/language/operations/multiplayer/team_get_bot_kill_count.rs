use language::operations::Operation;

pub struct TeamGetBotKillCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 450;

pub const IDENT: &str = "team_get_bot_kill_count";

impl Operation for TeamGetBotKillCountOp {
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
