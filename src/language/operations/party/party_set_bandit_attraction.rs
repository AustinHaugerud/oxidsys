use language::operations::Operation;

pub struct PartySetBanditAttractionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1645;

pub const IDENT: &str = "party_set_bandit_attraction";

impl Operation for PartySetBanditAttractionOp {
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
