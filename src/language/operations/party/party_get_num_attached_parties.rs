use language::operations::Operation;

pub struct PartyGetNumAttachedPartiesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1695;

pub const IDENT: &str = "party_get_num_attached_parties";

impl Operation for PartyGetNumAttachedPartiesOp {
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
