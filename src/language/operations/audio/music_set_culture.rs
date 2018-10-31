use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MusicSetCultureOp;

const DOC : &str = "Sets current culture(s) in the game (see mtf_* flags in header_music.py for reference) so the game engine can pick matching tracks from module_music.py. Use 0 to stop any currently playing music (it will resume when cultures are later set to something).";

pub const OP_CODE: u32 = 604;

pub const IDENT: &str = "music_set_culture";

impl Operation for MusicSetCultureOp {
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
            param_docs: vec![make_param_doc("<culture_type>", "")],
        }
    }
}
