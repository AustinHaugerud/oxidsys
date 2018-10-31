use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyCollectAttachmentsToPartyOp;

const DOC : &str = "Mostly used in various battle and AI calculations. Will create an aggregate party from all parties attached to the source party.";

pub const OP_CODE: u32 = 1662;

pub const IDENT: &str = "party_collect_attachments_to_party";

impl Operation for PartyCollectAttachmentsToPartyOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<source_party_id>", ""),
                make_param_doc("<collected_party_id>", ""),
            ],
        }
    }
}
