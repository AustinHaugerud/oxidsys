use language::operations::Operation;

pub struct PartyPrisonerStackGetSizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1657;

pub const IDENT: &str = "party_prisoner_stack_get_size";

impl Operation for PartyPrisonerStackGetSizeOp {
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
