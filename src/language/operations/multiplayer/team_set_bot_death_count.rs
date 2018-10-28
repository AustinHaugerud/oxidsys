use language::operations::Operation;

pub struct TeamSetBotDeathCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 453;

pub const IDENT: &str = "team_set_bot_death_count";

impl Operation for TeamSetBotDeathCountOp {
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
