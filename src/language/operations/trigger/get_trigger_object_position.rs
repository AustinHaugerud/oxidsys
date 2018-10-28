use language::operations::Operation;

pub struct GetTriggerObjectPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 702;

pub const IDENT: &str = "get_trigger_object_position";

impl Operation for GetTriggerObjectPositionOp {
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
