use language::operations::Operation;

pub struct PartyGetAttachedPartyWithRankOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1696;

pub const IDENT: &str = "party_get_attached_party_with_rank";

impl Operation for PartyGetAttachedPartyWithRankOp {
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
