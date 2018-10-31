use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenMissionOp;

const DOC: &str = "Starts the mission, using previously defined scene and mission template.";

pub const OP_CODE: u32 = 2048;

pub const IDENT: &str = "change_screen_mission";

impl Operation for ChangeScreenMissionOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
