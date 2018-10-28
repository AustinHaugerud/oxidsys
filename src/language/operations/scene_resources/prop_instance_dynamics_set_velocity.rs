use language::operations::Operation;

pub struct PropInstanceDynamicsSetVelocityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1872;

pub const IDENT: &str = "prop_instance_dynamics_set_velocity";

impl Operation for PropInstanceDynamicsSetVelocityOp {
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
