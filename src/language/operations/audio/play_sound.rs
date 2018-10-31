use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlaySoundOp;

const DOC : &str = "Plays a sound. If the operation is called from agent, scene_prop or item trigger, then the sound will be positional and 3D. See sf_* flags in header_sounds.py for reference on possible options.";

pub const OP_CODE: u32 = 600;

pub const IDENT: &str = "play_sound";

impl Operation for PlaySoundOp {
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
                make_param_doc("<sound_id>", ""),
                make_param_doc("[options]", ""),
            ],
        }
    }
}
