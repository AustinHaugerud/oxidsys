use language::operations::Operation;

pub struct SetMerchandiseMaxValueOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1491;

pub const IDENT: &str = "set_merchandise_max_value";

impl Operation for SetMerchandiseMaxValueOp {
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
