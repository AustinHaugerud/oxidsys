use language::operations::Operation;

pub struct PlayerSetDeathCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 436;

pub const IDENT: &str = "player_set_death_count";

impl Operation for PlayerSetDeathCountOp {
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
