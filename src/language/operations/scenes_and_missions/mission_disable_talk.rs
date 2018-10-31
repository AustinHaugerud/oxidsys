use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionDisableTalkOp;

const DOC: &str = "Disables dialogue with agents on the scene.";

pub const OP_CODE: u32 = 1936;

pub const IDENT: &str = "mission_disable_talk";

impl Operation for MissionDisableTalkOp {
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
