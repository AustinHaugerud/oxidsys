use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauSetAmbientLightOp;

const DOC : &str = "Not documented. Used for tableaus rendered from 3D objects to provide uniform tinted lighting.";

pub const OP_CODE: u32 = 1987;

pub const IDENT: &str = "cur_tableau_set_ambient_light";

impl Operation for CurTableauSetAmbientLightOp {
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
                make_param_doc("<red_fixed_point>", ""),
                make_param_doc("<green_fixed_point>", ""),
                make_param_doc("<blue_fixed_point>", ""),
            ],
        }
    }
}
