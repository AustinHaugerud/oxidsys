use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstancePlaySoundOp;

const DOC : &str = "Version 1.153+. Makes the scene prop play a specified sound. See sf_* flags in header_sounds.py for reference on possible options.";

pub const OP_CODE: u32 = 1881;

pub const IDENT: &str = "prop_instance_play_sound";

impl Operation for PropInstancePlaySoundOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<scene_prop_id>", ""),
                make_param_doc("<sound_id>", ""),
                make_param_doc("[flags]", ""),
            ],
        }
    }
}
