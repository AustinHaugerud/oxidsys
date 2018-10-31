use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetStartupSunLightOp;

const DOC: &str = "Version 1.153+. Defines the sunlight color for the scene.";

pub const OP_CODE: u32 = 2390;

pub const IDENT: &str = "set_startup_sun_light";

impl Operation for SetStartupSunLightOp {
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
