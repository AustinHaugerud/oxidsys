use language::operations::Operation;

pub struct ChangeScreenGiveMembersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2056;

pub const IDENT: &str = "change_screen_give_members";

impl Operation for ChangeScreenGiveMembersOp {
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
