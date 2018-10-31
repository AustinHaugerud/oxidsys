use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetStartupGroundAmbientLightOp;

const DOC: &str = "Version 1.153+. Defines the ambient light color for the ground.";

pub const OP_CODE: u32 = 2392;

pub const IDENT: &str = "set_startup_ground_ambient_light";

impl Operation for SetStartupGroundAmbientLightOp {
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
                make_param_doc("<r>", ""),
                make_param_doc("<g>", ""),
                make_param_doc("<b>", ""),
            ],
        }
    }
}
