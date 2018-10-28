use language::operations::Operation;

pub struct RemoveTroopFromSiteOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1251;

pub const IDENT: &str = "remove_troop_from_site";

impl Operation for RemoveTroopFromSiteOp {
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
