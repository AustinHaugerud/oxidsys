use language::operations::Operation;

pub struct TeamGetScoreOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 455;

pub const IDENT: &str = "team_get_score";

impl Operation for TeamGetScoreOp {
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
