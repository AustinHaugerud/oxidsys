use language::operations::Operation;

pub struct TeamGetBotDeathCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 452;

pub const IDENT: &str = "team_get_bot_death_count";

impl Operation for TeamGetBotDeathCountOp {
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
