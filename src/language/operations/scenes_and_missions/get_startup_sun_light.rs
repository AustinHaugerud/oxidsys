use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetStartupSunLightOp;

const DOC: &str =
    "Version 1.165+. Returns startup sunlight color in (x, y, z) coordinates of position register.";

pub const OP_CODE: u32 = 2394;

pub const IDENT: &str = "get_startup_sun_light";

impl Operation for GetStartupSunLightOp {
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
            param_docs: vec![make_param_doc("<position_no>", "")],
        }
    }
}
