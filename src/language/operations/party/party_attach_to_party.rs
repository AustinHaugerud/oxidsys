use language::operations::Operation;

pub struct PartyAttachToPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1660;

pub const IDENT: &str = "party_attach_to_party";

impl Operation for PartyAttachToPartyOp {
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
