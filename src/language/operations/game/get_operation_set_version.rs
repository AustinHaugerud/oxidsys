use language::operations::Operation;

pub struct GetOperationSetVersionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 55;

pub const IDENT: &str = "get_operation_set_version";

impl Operation for GetOperationSetVersionOp {
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
