use language::operations::Operation;

pub struct RebuildShadowMapOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2393;

pub const IDENT: &str = "rebuild_shadow_map";

impl Operation for RebuildShadowMapOp {
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
