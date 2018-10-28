use language::operations::Operation;

pub struct PartyStackGetTroopDnaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1655;

pub const IDENT: &str = "party_stack_get_troop_dna";

impl Operation for PartyStackGetTroopDnaOp {
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
