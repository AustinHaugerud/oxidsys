use language::operations::Operation;

pub struct TroopsCanJoinAsPrisonerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 106;

pub const IDENT: &str = "troops_can_join_as_prisoner";

impl Operation for TroopsCanJoinAsPrisonerOp {
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
