use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ParticleSystemEmitOp;

const DOC : &str = "Adds a particle system in some fancy way. Uses position offset and color provided to previous calls to (set_position_delta) and (set_current_color). Can only be used in item/prop triggers.";

pub const OP_CODE: u32 = 1968;

pub const IDENT: &str = "particle_system_emit";

impl Operation for ParticleSystemEmitOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<par_sys_id>", ""),
                make_param_doc("<value_num_particles>", ""),
                make_param_doc("<value_period>", ""),
            ],
        }
    }
}
