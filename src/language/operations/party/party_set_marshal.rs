use language::operations::Operation;

pub struct PartySetMarshalOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1604;

pub const IDENT: &str = "party_set_marshal";

impl Operation for PartySetMarshalOp {
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
