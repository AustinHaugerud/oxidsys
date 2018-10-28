use language::operations::Operation;

pub struct PropInstanceStopAllParticleSystemsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1887;

pub const IDENT: &str = "prop_instance_stop_all_particle_systems";

impl Operation for PropInstanceStopAllParticleSystemsOp {
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
