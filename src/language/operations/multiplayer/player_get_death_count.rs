use language::operations::Operation;

pub struct PlayerGetDeathCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 435;

pub const IDENT: &str = "player_get_death_count";

impl Operation for PlayerGetDeathCountOp {
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
