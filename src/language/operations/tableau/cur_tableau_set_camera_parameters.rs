use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauSetCameraParametersOp;

const DOC : &str = "Not documented. Used to define camera parameters for tableau rendering. Perspective camera is generally used to render 3D objects for tableaus, while non-perspective camera is used to modify tableau texture meshes.";

pub const OP_CODE: u32 = 1989;

pub const IDENT: &str = "cur_tableau_set_camera_parameters";

impl Operation for CurTableauSetCameraParametersOp {
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
            num_required: 5,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<is_perspective>", ""),
                make_param_doc("<camera_width_times_1000>", ""),
                make_param_doc("<camera_height_times_1000>", ""),
                make_param_doc("<camera_near_times_1000>", ""),
                make_param_doc("<camera_far_times_1000>", ""),
            ],
        }
    }
}
