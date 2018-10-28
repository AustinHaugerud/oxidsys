use language::operations::Operation;

pub struct RemoveRegularPrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1211;

pub const IDENT: &str = "remove_regular_prisoners";

impl Operation for RemoveRegularPrisonersOp {
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
