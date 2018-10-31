use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentPlaySoundOp;

const DOC: &str = "Makes the agent emit the specified sound.";

pub const OP_CODE: u32 = 1750;

pub const IDENT: &str = "agent_play_sound";

impl Operation for AgentPlaySoundOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<sound_id>", ""),
            ],
        }
    }
}
