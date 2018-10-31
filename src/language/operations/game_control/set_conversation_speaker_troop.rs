use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetConversationSpeakerTroopOp;

const DOC : &str = "Allows to dynamically switch speaking troops during the dialog when developer doesn't know in advance who will be doing the speaking. Should be placed in post-talk code section of dialog entry.";

pub const OP_CODE: u32 = 2197;

pub const IDENT: &str = "set_conversation_speaker_troop";

impl Operation for SetConversationSpeakerTroopOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
