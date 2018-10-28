use language::operations::Operation;

pub struct TeamGetLeaderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1792;

pub const IDENT: &str = "team_get_leader";

impl Operation for TeamGetLeaderOp {
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
