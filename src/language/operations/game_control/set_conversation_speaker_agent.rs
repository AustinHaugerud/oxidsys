use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetConversationSpeakerAgentOp;

const DOC : &str = "Allows to dynamically switch speaking agents during the dialog when developer doesn't know in advance who will be doing the speaking. Should be placed in post-talk code section of dialog entry.";

pub const OP_CODE: u32 = 2198;

pub const IDENT: &str = "set_conversation_speaker_agent";

impl Operation for SetConversationSpeakerAgentOp {
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
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}
