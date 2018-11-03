use language::operations::{Operation};
pub mod dialog_box;
pub mod display_debug_message;
pub mod display_log_message;
pub mod display_message;
pub mod question_box;
pub mod set_show_messages;
pub mod tutorial_box;
pub mod tutorial_message;
pub mod tutorial_message_set_background;
pub mod tutorial_message_set_center_justify;
pub mod tutorial_message_set_position;
pub mod tutorial_message_set_size;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(dialog_box::DialogBoxOp {}));
    result.push(Box::new(display_debug_message::DisplayDebugMessageOp {}));
    result.push(Box::new(display_log_message::DisplayLogMessageOp {}));
    result.push(Box::new(display_message::DisplayMessageOp {}));
    result.push(Box::new(question_box::QuestionBoxOp {}));
    result.push(Box::new(set_show_messages::SetShowMessagesOp {}));
    result.push(Box::new(tutorial_box::TutorialBoxOp {}));
    result.push(Box::new(tutorial_message::TutorialMessageOp {}));
    result.push(Box::new(
        tutorial_message_set_background::TutorialMessageSetBackgroundOp {},
    ));
    result.push(Box::new(
        tutorial_message_set_center_justify::TutorialMessageSetCenterJustifyOp {},
    ));
    result.push(Box::new(
        tutorial_message_set_position::TutorialMessageSetPositionOp {},
    ));
    result.push(Box::new(
        tutorial_message_set_size::TutorialMessageSetSizeOp {},
    ));
    result
}
