use language::operations::Operation;

pub struct PartyCanJoinAsPrisonerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 104;

pub const IDENT: &str = "party_can_join_as_prisoner";

impl Operation for PartyCanJoinAsPrisonerOp {
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
