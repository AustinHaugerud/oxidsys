use language::operations::Operation;

pub struct StoreNumRegularPrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2159;

pub const IDENT: &str = "store_num_regular_prisoners";

impl Operation for StoreNumRegularPrisonersOp {
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
