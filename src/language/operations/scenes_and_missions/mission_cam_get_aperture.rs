use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamGetApertureOp;

const DOC: &str = "Not documented. View angle?";

pub const OP_CODE: u32 = 2013;

pub const IDENT: &str = "mission_cam_get_aperture";

impl Operation for MissionCamGetApertureOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
