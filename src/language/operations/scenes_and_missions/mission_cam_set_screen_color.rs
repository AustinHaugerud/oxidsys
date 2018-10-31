use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionCamSetScreenColorOp;

const DOC : &str = "Not documented. Paints the screen with solid color. Parameter <value> contains color code with alpha component. Can be used to block screen entirely, add tint etc.";

pub const OP_CODE: u32 = 2008;

pub const IDENT: &str = "mission_cam_set_screen_color";

impl Operation for MissionCamSetScreenColorOp {
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
