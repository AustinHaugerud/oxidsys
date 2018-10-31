use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreLastSoundChannelOp;

const DOC: &str =
    "Version 1.153+. UNTESTED. Stores the sound channel used for the last sound operation.";

pub const OP_CODE: u32 = 615;

pub const IDENT: &str = "store_last_sound_channel";

impl Operation for StoreLastSoundChannelOp {
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
