use language::operations::Operation;

pub struct PartyJoinAsPrisonerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1202;

pub const IDENT: &str = "party_join_as_prisoner";

impl Operation for PartyJoinAsPrisonerOp {
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
