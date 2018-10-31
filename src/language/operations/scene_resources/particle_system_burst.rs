use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ParticleSystemBurstOp;

const DOC: &str = "Bursts a particle system in specified position.";

pub const OP_CODE: u32 = 1969;

pub const IDENT: &str = "particle_system_burst";

impl Operation for ParticleSystemBurstOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("[percentage_burst_strength]", ""),
            ],
        }
    }
}
