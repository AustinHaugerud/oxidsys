use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlaySoundAtPositionOp;

const DOC : &str = "Plays a sound in specified scene position. See sf_* flags in header_sounds.py for reference on possible options.";

pub const OP_CODE: u32 = 599;

pub const IDENT: &str = "play_sound_at_position";

impl Operation for PlaySoundAtPositionOp {
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
                make_param_doc("<sound_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("[options]", ""),
            ],
        }
    }
}
