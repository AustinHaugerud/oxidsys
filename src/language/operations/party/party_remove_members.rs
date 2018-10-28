use language::operations::Operation;

pub struct PartyRemoveMembersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1615;

pub const IDENT: &str = "party_remove_members";

impl Operation for PartyRemoveMembersOp {
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
