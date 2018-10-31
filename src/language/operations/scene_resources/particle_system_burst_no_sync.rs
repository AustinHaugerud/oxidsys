use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ParticleSystemBurstNoSyncOp;

const DOC : &str = "Version 1.153+. Same as above, but apparently does not synchronize this between server and client.";

pub const OP_CODE: u32 = 1975;

pub const IDENT: &str = "particle_system_burst_no_sync";

impl Operation for ParticleSystemBurstNoSyncOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<par_sys_id>", ""),
                make_param_doc("<position_no>", ""),
                make_param_doc("[percentage_burst_strength]", ""),
            ],
        }
    }
}
