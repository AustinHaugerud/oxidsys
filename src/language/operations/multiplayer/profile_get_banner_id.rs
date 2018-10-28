use language::operations::Operation;

pub struct ProfileGetBannerIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 350;

pub const IDENT: &str = "profile_get_banner_id";

impl Operation for ProfileGetBannerIdOp {
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
