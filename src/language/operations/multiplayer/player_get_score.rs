use language::operations::Operation;

pub struct PlayerGetScoreOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 431;

pub const IDENT: &str = "player_get_score";

impl Operation for PlayerGetScoreOp {
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
