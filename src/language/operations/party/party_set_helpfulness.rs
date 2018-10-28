use language::operations::Operation;

pub struct PartySetHelpfulnessOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1647;

pub const IDENT: &str = "party_set_helpfulness";

impl Operation for PartySetHelpfulnessOp {
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
