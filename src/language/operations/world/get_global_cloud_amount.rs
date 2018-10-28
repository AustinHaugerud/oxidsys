use language::operations::Operation;

pub struct GetGlobalCloudAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 90;

pub const IDENT: &str = "get_global_cloud_amount";

impl Operation for GetGlobalCloudAmountOp {
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
