use language::operations::Operation;

pub struct PartySetAiInitiativeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1639;

pub const IDENT: &str = "party_set_ai_initiative";

impl Operation for PartySetAiInitiativeOp {
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
