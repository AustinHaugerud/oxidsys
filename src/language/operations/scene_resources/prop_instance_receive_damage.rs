use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceReceiveDamageOp;

const DOC : &str = "Makes scene prop instance receive specified amount of damage from any arbitrary agent. Agent reference is apparently necessary to properly initialize ti_on_scene_prop_hit trigger parameters.";

pub const OP_CODE: u32 = 1877;

pub const IDENT: &str = "prop_instance_receive_damage";

impl Operation for PropInstanceReceiveDamageOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<damage_value>", ""),
            ],
        }
    }
}
