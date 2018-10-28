use language::operations::Operation;

pub struct StoreNumPartiesOfTemplateOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2310;

pub const IDENT: &str = "store_num_parties_of_template";

impl Operation for StoreNumPartiesOfTemplateOp {
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
