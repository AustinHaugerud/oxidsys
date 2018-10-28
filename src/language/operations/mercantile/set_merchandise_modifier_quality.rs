use language::operations::Operation;

pub struct SetMerchandiseModifierQualityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1490;

pub const IDENT: &str = "set_merchandise_modifier_quality";

impl Operation for SetMerchandiseModifierQualityOp {
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
