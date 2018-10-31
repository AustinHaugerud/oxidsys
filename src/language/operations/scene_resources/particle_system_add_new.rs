use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ParticleSystemAddNewOp;

const DOC : &str = "Adds a new particle system to an object. Uses position offset and color provided to previous calls to (set_position_delta) and (set_current_color). Can only be used in item/prop triggers.";

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

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 1,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<par_sys_id>", ""),
                make_param_doc("[position]", ""),
            ],
        }
    }
}
