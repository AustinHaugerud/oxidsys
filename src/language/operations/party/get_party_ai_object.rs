use language::operations::Operation;

pub struct GetPartyAiObjectOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2291;

pub const IDENT: &str = "get_party_ai_object";

impl Operation for GetPartyAiObjectOp {
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
