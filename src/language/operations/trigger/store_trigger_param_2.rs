use language::operations::Operation;

pub struct StoreTriggerParam2Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2072;

pub const IDENT: &str = "store_trigger_param_2";

impl Operation for StoreTriggerParam2Op {
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
