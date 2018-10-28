use language::operations::Operation;

pub struct ParticleSystemBurstOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1969;

pub const IDENT: &str = "particle_system_burst";

impl Operation for ParticleSystemBurstOp {
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
