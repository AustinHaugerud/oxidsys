use language::operations::Operation;

pub struct PartyTemplateSetSlotOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 504;

pub const IDENT: &str = "party_template_set_slot";

impl Operation for PartyTemplateSetSlotOp {
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
