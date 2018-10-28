use language::operations::Operation;

pub struct PlayerSetScoreOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 432;

pub const IDENT: &str = "player_set_score";

impl Operation for PlayerSetScoreOp {
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
