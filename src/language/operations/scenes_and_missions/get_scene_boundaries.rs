use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetSceneBoundariesOp;

const DOC : &str = "Retrieves the coordinates of the top-left and bottom-right corner of the scene to the provided position registers.";

pub const OP_CODE: u32 = 1799;

pub const IDENT: &str = "get_scene_boundaries";

impl Operation for GetSceneBoundariesOp {
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
                make_param_doc("<position_min>", ""),
                make_param_doc("<position_max>", ""),
            ],
        }
    }
}
