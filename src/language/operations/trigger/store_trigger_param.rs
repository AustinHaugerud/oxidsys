use language::operations::Operation;

pub struct StoreTriggerParamOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2070;

pub const IDENT: &str = "store_trigger_param";

impl Operation for StoreTriggerParamOp {
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
