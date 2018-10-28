use language::operations::Operation;

pub struct PartyClearParticleSystemsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1679;

pub const IDENT: &str = "party_clear_particle_systems";

impl Operation for PartyClearParticleSystemsOp {
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
