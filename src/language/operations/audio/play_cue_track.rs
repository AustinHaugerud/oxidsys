use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayCueTrackOp;

const DOC : &str = "Plays specified music track OVER any currently played music track (so you can get two music tracks playing simultaneously). Hardly useful.";

pub const OP_CODE: u32 = 602;

pub const IDENT: &str = "play_cue_track";

impl Operation for PlayCueTrackOp {
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
            param_docs: vec![make_param_doc("<track_id>", "")],
        }
    }
}
