use language::operations::Operation;

pub struct AgentPlaySoundOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1750;

pub const IDENT: &str = "agent_play_sound";

impl Operation for AgentPlaySoundOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
