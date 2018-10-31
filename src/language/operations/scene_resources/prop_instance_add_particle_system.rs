use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceAddParticleSystemOp;

const DOC : &str = "Version 1.153+. Adds a new particle system to the scene prop. Note that <position_no> is local, i.e. in relation to scene prop's coordinates and rotation.";

pub const OP_CODE: u32 = 1886;

pub const IDENT: &str = "prop_instance_add_particle_system";

impl Operation for PropInstanceAddParticleSystemOp {
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
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<par_sys_id>", ""),
                make_param_doc("<position_no>", ""),
            ],
        }
    }
}
