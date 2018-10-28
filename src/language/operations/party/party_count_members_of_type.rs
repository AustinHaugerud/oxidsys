use language::operations::Operation;

pub struct PartyCountMembersOfTypeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1630;

pub const IDENT: &str = "party_count_members_of_type";

impl Operation for PartyCountMembersOfTypeOp {
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
