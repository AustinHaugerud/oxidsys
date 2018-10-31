use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceStopSoundOp;

const DOC: &str = "Version 1.153+. Stops any sound currently played by the scene prop instance.";

pub const OP_CODE: u32 = 1882;

pub const IDENT: &str = "prop_instance_stop_sound";

impl Operation for PropInstanceStopSoundOp {
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
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
