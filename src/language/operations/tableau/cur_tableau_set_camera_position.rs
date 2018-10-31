use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauSetCameraPositionOp;

const DOC : &str = "Not documented. Used for tableaus rendered from 3D objects to position camera as necessary (usually with a perspective camera).";

pub const OP_CODE: u32 = 1988;

pub const IDENT: &str = "cur_tableau_set_camera_position";

impl Operation for CurTableauSetCameraPositionOp {
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
            param_docs: vec![make_param_doc("<position>", "")],
        }
    }
}
