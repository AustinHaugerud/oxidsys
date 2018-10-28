use language::operations::Operation;

pub struct StoreTriggerParam1Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2071;

pub const IDENT: &str = "store_trigger_param_1";

impl Operation for StoreTriggerParam1Op {
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
