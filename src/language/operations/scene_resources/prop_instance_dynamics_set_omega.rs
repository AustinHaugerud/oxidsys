use language::operations::Operation;

pub struct PropInstanceDynamicsSetOmegaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1873;

pub const IDENT: &str = "prop_instance_dynamics_set_omega";

impl Operation for PropInstanceDynamicsSetOmegaOp {
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
