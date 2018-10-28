use language::operations::Operation;

pub struct PartyTemplateSlotGeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 564;

pub const IDENT: &str = "party_template_slot_ge";

impl Operation for PartyTemplateSlotGeOp {
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
