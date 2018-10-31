use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateSimpleTextBoxOverlayOp;

const DOC : &str = "Creates a text field overlay, where user can enter any text. Returns text field's overlay_id. Text contents of the field can be retrieved from s0 trigger in ti_on_presentation_event_state_change event for the text field.";

pub const OP_CODE: u32 = 919;

pub const IDENT: &str = "create_simple_text_box_overlay";

impl Operation for CreateSimpleTextBoxOverlayOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
