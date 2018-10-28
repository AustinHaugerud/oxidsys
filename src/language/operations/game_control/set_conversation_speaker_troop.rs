use language::operations::Operation;

pub struct SetConversationSpeakerTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2197;

pub const IDENT: &str = "set_conversation_speaker_troop";

impl Operation for SetConversationSpeakerTroopOp {
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
