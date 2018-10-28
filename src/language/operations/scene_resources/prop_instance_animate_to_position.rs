use language::operations::Operation;

pub struct PropInstanceAnimateToPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1860;

pub const IDENT: &str = "prop_instance_animate_to_position";

impl Operation for PropInstanceAnimateToPositionOp {
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
