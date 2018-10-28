use language::operations::Operation;

pub struct PartyCountPrisonersOfTypeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1632;

pub const IDENT: &str = "party_count_prisoners_of_type";

impl Operation for PartyCountPrisonersOfTypeOp {
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
