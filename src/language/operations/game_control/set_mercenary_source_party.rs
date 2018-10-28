use language::operations::Operation;

pub struct SetMercenarySourcePartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1320;

pub const IDENT: &str = "set_mercenary_source_party";

impl Operation for SetMercenarySourcePartyOp {
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
