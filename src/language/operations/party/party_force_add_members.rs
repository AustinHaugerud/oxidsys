use language::operations::Operation;

pub struct PartyForceAddMembersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1613;

pub const IDENT: &str = "party_force_add_members";

impl Operation for PartyForceAddMembersOp {
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
