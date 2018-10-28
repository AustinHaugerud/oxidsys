use language::operations::Operation;

pub struct SetGlobalCloudAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 91;

pub const IDENT: &str = "set_global_cloud_amount";

impl Operation for SetGlobalCloudAmountOp {
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
