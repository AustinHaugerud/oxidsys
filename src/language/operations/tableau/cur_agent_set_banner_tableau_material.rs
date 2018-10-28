use language::operations::Operation;

pub struct CurAgentSetBannerTableauMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1986;

pub const IDENT: &str = "cur_agent_set_banner_tableau_material";

impl Operation for CurAgentSetBannerTableauMaterialOp {
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
