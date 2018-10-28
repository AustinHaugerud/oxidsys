use language::operations::Operation;

pub struct PartyCollectAttachmentsToPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1662;

pub const IDENT: &str = "party_collect_attachments_to_party";

impl Operation for PartyCollectAttachmentsToPartyOp {
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
