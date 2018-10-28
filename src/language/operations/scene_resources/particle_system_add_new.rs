use language::operations::Operation;

pub struct ParticleSystemAddNewOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1965;

pub const IDENT: &str = "particle_system_add_new";

impl Operation for ParticleSystemAddNewOp {
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
