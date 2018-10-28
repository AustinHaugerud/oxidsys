use language::operations::Operation;

pub struct SetTriggerResultOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2075;

pub const IDENT: &str = "set_trigger_result";

impl Operation for SetTriggerResultOp {
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
