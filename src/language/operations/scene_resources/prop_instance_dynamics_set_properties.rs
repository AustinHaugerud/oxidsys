use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceDynamicsSetPropertiesOp;

const DOC : &str = "Initializes physical parameters of a scene prop. Position (X,Y) coordinates are used to store object's mass and friction coefficient. Coordinate Z is reserved (set it to zero just in case). Scene prop must be defined as sokf_moveable|sokf_dynamic_physics, and a call to (prop_instance_enable_physics) must be previously made.";

pub const OP_CODE: u32 = 1871;

pub const IDENT: &str = "prop_instance_dynamics_set_properties";

impl Operation for PropInstanceDynamicsSetPropertiesOp {
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
