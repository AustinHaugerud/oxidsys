use language::operations::Operation;

pub struct StoreTriggerParam3Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2073;

pub const IDENT: &str = "store_trigger_param_3";

impl Operation for StoreTriggerParam3Op {
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
