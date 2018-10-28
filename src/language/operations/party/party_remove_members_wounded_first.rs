use language::operations::Operation;

pub struct PartyRemoveMembersWoundedFirstOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1619;

pub const IDENT: &str = "party_remove_members_wounded_first";

impl Operation for PartyRemoveMembersWoundedFirstOp {
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
