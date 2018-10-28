use language::operations::Operation;

pub struct ParticleSystemBurstNoSyncOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1975;

pub const IDENT: &str = "particle_system_burst_no_sync";

impl Operation for ParticleSystemBurstNoSyncOp {
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
