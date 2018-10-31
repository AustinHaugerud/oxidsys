use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamAnimateToScreenColorOp;

const DOC : &str = "Not documented. Same as above, but color change is gradual. Used in Native to fill the screen with white before the end of marriage scene.";

pub const OP_CODE: u32 = 2009;

pub const IDENT: &str = "mission_cam_animate_to_screen_color";

impl Operation for MissionCamAnimateToScreenColorOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
