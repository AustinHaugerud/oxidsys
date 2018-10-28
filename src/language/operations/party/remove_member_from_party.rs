use language::operations::Operation;

pub struct RemoveMemberFromPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1210;

pub const IDENT: &str = "remove_member_from_party";

impl Operation for RemoveMemberFromPartyOp {
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
