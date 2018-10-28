use language::operations::Operation;

pub struct PartyAddTemplateOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1675;

pub const IDENT: &str = "party_add_template";

impl Operation for PartyAddTemplateOp {
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
