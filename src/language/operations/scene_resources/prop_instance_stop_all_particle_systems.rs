use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceStopAllParticleSystemsOp;

const DOC: &str =
    "Version 1.153+. Removes all particle systems currently associated with scene prop instance.";

pub const OP_CODE: u32 = 1887;

pub const IDENT: &str = "prop_instance_stop_all_particle_systems";

impl Operation for PropInstanceStopAllParticleSystemsOp {
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
            num_optional: 0,
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
