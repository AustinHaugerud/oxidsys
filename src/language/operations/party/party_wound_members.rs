use language::operations::Operation;

pub struct PartyWoundMembersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1618;

pub const IDENT: &str = "party_wound_members";

impl Operation for PartyWoundMembersOp {
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
