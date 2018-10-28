use language::operations::Operation;

pub struct PropInstanceDynamicsApplyImpulseOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1874;

pub const IDENT: &str = "prop_instance_dynamics_apply_impulse";

impl Operation for PropInstanceDynamicsApplyImpulseOp {
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
