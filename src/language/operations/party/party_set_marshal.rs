use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetMarshalOp;

const DOC : &str = "Sets party as a marshall party or turns it back to normal party. Value is either 1 or 0. This affects party behavior, but exact effects are not known. Alternative operation name spelling added to enable compatibility with Viking Conquest DLC module system.";

pub const OP_CODE: u32 = 1604;

pub const IDENT: &str = "party_set_marshal";

impl Operation for PartySetMarshalOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
