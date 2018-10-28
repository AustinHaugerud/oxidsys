use language::operations::Operation;

pub struct TeamSetScoreOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 456;

pub const IDENT: &str = "team_set_score";

impl Operation for TeamSetScoreOp {
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
