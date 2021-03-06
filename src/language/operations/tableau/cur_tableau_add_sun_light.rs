use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddSunLightOp;

const DOC : &str = "Not documented. Typically used for tableaus rendered from 3D objects to add a directional light source. Note that position coordinates do not matter, only rotation (i.e. light rays direction) does.";

pub const OP_CODE: u32 = 1991;

pub const IDENT: &str = "cur_tableau_add_sun_light";

impl Operation for CurTableauAddSunLightOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<position>", ""),
                make_param_doc("<red_fixed_point>", ""),
                make_param_doc("<green_fixed_point>", ""),
                make_param_doc("<blue_fixed_point>", ""),
            ],
        }
    }
}
