use language::operations::{Operation, ParamInfo};

pub struct MissionEnableTalkOp;

const DOC: &str = "Allows dialogue with agents on the scene.";

pub const OP_CODE: u32 = 1935;

pub const IDENT: &str = "mission_enable_talk";

impl Operation for MissionEnableTalkOp {
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
