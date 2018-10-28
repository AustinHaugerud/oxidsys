use language::operations::Operation;

pub struct PropInstanceInitializeRotationAnglesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1866;

pub const IDENT: &str = "prop_instance_initialize_rotation_angles";

impl Operation for PropInstanceInitializeRotationAnglesOp {
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
