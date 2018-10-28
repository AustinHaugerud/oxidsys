use language::operations::Operation;

pub struct Store01RandomPartiesInRangeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2255;

pub const IDENT: &str = "store01_random_parties_in_range";

impl Operation for Store01RandomPartiesInRangeOp {
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
