use language::operations::Operation;

pub struct PartySetAiObjectOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1641;

pub const IDENT: &str = "party_set_ai_object";

impl Operation for PartySetAiObjectOp {
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
