use language::operations::Operation;

pub struct PropInstanceAddParticleSystemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1886;

pub const IDENT: &str = "prop_instance_add_particle_system";

impl Operation for PropInstanceAddParticleSystemOp {
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
