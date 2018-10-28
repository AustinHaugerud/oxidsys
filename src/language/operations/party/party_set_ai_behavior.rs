use language::operations::Operation;

pub struct PartySetAiBehaviorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1640;

pub const IDENT: &str = "party_set_ai_behavior";

impl Operation for PartySetAiBehaviorOp {
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
