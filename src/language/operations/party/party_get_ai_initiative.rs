use language::operations::Operation;

pub struct PartyGetAiInitiativeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1638;

pub const IDENT: &str = "party_get_ai_initiative";

impl Operation for PartyGetAiInitiativeOp {
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
