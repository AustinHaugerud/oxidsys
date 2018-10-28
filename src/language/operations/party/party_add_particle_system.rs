use language::operations::Operation;

pub struct PartyAddParticleSystemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1678;

pub const IDENT: &str = "party_add_particle_system";

impl Operation for PartyAddParticleSystemOp {
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
