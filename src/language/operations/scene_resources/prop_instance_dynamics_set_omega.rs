use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceDynamicsSetOmegaOp;

const DOC : &str = "Sets current rotation speed for a scene prop. Position's coordinates define rotational speed around corresponding axis. Same comments as for (prop_instance_dynamics_set_properties).";

pub const OP_CODE: u32 = 1873;

pub const IDENT: &str = "prop_instance_dynamics_set_omega";

impl Operation for PropInstanceDynamicsSetOmegaOp {
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
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}
