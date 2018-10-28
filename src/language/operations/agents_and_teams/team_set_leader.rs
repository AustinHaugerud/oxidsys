use language::operations::Operation;

pub struct TeamSetLeaderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1793;

pub const IDENT: &str = "team_set_leader";

impl Operation for TeamSetLeaderOp {
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
