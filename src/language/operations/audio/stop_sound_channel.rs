use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StopSoundChannelOp;

const DOC: &str = "Version 1.153+. UNTESTED. Stops sound playing on specified sound channel.";

pub const OP_CODE: u32 = 616;

pub const IDENT: &str = "stop_sound_channel";

impl Operation for StopSoundChannelOp {
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
            param_docs: vec![make_param_doc("<sound_channel_no>", "")],
        }
    }
}
