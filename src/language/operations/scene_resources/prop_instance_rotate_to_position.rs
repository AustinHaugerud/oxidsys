use language::operations::Operation;

pub struct PropInstanceRotateToPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1865;

pub const IDENT: &str = "prop_instance_rotate_to_position";

impl Operation for PropInstanceRotateToPositionOp {
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
