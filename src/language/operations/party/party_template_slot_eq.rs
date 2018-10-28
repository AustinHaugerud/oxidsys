use language::operations::Operation;

pub struct PartyTemplateSlotEqOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 544;

pub const IDENT: &str = "party_template_slot_eq";

impl Operation for PartyTemplateSlotEqOp {
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
