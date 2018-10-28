use language::operations::Operation;

pub struct PartyGetTemplateIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1609;

pub const IDENT: &str = "party_get_template_id";

impl Operation for PartyGetTemplateIdOp {
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
