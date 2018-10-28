use language::operations::Operation;

pub struct PartyRelocateNearPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1623;

pub const IDENT: &str = "party_relocate_near_party";

impl Operation for PartyRelocateNearPartyOp {
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
