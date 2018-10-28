use language::operations::Operation;

pub struct DistributePartyAmongPartyGroupOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1698;

pub const IDENT: &str = "distribute_party_among_party_group";

impl Operation for DistributePartyAmongPartyGroupOp {
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
