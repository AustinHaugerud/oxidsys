use language::operations::Operation;

pub struct TroopJoinAsPrisonerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1204;

pub const IDENT: &str = "troop_join_as_prisoner";

impl Operation for TroopJoinAsPrisonerOp {
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
