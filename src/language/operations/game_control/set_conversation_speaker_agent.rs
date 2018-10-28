use language::operations::Operation;

pub struct SetConversationSpeakerAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2198;

pub const IDENT: &str = "set_conversation_speaker_agent";

impl Operation for SetConversationSpeakerAgentOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
