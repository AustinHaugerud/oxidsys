use language::operations::Operation;

pub struct PartyAddXpToStackOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1670;

pub const IDENT: &str = "party_add_xp_to_stack";

impl Operation for PartyAddXpToStackOp {
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
