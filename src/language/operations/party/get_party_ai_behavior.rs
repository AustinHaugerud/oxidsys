use language::operations::Operation;

pub struct GetPartyAiBehaviorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2290;

pub const IDENT: &str = "get_party_ai_behavior";

impl Operation for GetPartyAiBehaviorOp {
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
