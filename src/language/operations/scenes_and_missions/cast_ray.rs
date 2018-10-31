use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CastRayOp;

const DOC : &str = "Version 1.161+. Casts a ray starting from <ray_position_register> and stores the closest hit position into <hit_position_register> (fails if no hits). If the body hit is a scene prop, its instance id will be stored into <destination>, otherwise it will be -1. Optional <ray_length> parameter seems to have no effect.";

pub const OP_CODE: u32 = 1900;

pub const IDENT: &str = "cast_ray";

impl Operation for CastRayOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<hit_position_register>", ""),
                make_param_doc("<ray_position_register>", ""),
                make_param_doc("[<ray_length_fixed_point>]", ""),
            ],
        }
    }
}
