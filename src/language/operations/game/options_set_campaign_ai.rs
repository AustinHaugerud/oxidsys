use language::operations::Operation;

pub struct OptionsSetCampaignAiOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 267;

pub const IDENT: &str = "options_set_campaign_ai";

impl Operation for OptionsSetCampaignAiOp {
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
