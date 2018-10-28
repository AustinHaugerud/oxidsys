use language::operations::Operation;

pub struct PropInstanceEnablePhysicsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1864;

pub const IDENT: &str = "prop_instance_enable_physics";

impl Operation for PropInstanceEnablePhysicsOp {
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
