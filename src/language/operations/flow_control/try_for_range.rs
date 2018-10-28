use language::operations::Operation;

pub struct TryForRangeOp;

const DOC: &str = r#"
Iterate over range [a,b).
Format: try_for_range <iterator> <inclusive_beg> <exclusive_end>;
"#;

const OP_CODE: u16 = 6;

const IDENT: &str = "try_for_range";

impl Operation for TryForRangeOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &str {
        DOC
    }

    fn identifier(&self) -> &str {
        IDENT
    }
}
