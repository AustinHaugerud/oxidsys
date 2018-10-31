use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayTrackOp;

const DOC : &str = "Plays specified music track. Possible options: 0 = finish current then play this, 1 = fade out current and start this, 2 = stop current abruptly and start this";

pub const OP_CODE: u32 = 601;

pub const IDENT: &str = "play_track";

impl Operation for PlayTrackOp {
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
                make_param_doc("<track_id>", ""),
                make_param_doc("[options]", ""),
            ],
        }
    }
}
