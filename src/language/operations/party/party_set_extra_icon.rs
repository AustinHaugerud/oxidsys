use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetExtraIconOp;

const DOC : &str = "Adds or removes an extra map icon to a party, possibly with some animations. Use -1 as map_icon_id to remove extra icon.";

pub const OP_CODE: u32 = 1682;

pub const IDENT: &str = "party_set_extra_icon";

impl Operation for PartySetExtraIconOp {
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
            num_required: 6,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<map_icon_id>", ""),
                make_param_doc("<vertical_offset_fixed_point>", ""),
                make_param_doc("<up_down_frequency_fixed_point>", ""),
                make_param_doc("<rotate_frequency_fixed_point>", ""),
                make_param_doc("<fade_in_out_frequency_fixed_point>", ""),
            ],
        }
    }
}
