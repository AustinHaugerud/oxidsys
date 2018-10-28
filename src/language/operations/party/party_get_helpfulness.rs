use language::operations::Operation;

pub struct PartyGetHelpfulnessOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1646;

pub const IDENT: &str = "party_get_helpfulness";

impl Operation for PartyGetHelpfulnessOp {
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
