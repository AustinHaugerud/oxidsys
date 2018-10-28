use language::operations::Operation;

pub struct PropInstanceDynamicsSetPropertiesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1871;

pub const IDENT: &str = "prop_instance_dynamics_set_properties";

impl Operation for PropInstanceDynamicsSetPropertiesOp {
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
