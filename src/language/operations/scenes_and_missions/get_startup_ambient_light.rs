use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetStartupAmbientLightOp;

const DOC : &str = "Version 1.165+. Returns startup ambient light color in (x, y, z) coordinates of position register.";

pub const OP_CODE: u32 = 2395;

pub const IDENT: &str = "get_startup_ambient_light";

impl Operation for GetStartupAmbientLightOp {
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
