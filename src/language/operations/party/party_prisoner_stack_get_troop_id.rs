use language::operations::Operation;

pub struct PartyPrisonerStackGetTroopIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1656;

pub const IDENT: &str = "party_prisoner_stack_get_troop_id";

impl Operation for PartyPrisonerStackGetTroopIdOp {
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
