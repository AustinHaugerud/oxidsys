use language::operations::Operation;

pub struct TryForRangeBackwardsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 7;

pub const IDENT: &str = "try_for_range_backwards";

impl Operation for TryForRangeBackwardsOp {
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
