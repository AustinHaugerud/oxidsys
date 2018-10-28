use language::operations::Operation;

pub struct TryForRangeBackwardsOp;

const DOC: &str = r#"
Iterate range [a,b) in reverse.
Format: try_for_range_backwards <iterator> <inclusive_begin> <exclusive_end>;
"#;

pub const OP_CODE: u16 = 7;

const IDENT: &str = r#"try_for_range_backwards"#;

impl Operation for TryForRangeBackwardsOp {
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
