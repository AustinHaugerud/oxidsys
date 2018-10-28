use language::operations::Operation;

pub struct PartyPrisonerStackGetTroopDnaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1658;

pub const IDENT: &str = "party_prisoner_stack_get_troop_dna";

impl Operation for PartyPrisonerStackGetTroopDnaOp {
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
