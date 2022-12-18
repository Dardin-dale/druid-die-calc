use rand::rng;

use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};


#[derive(Clone, Data, Lens)]
pub struct CalcState {
    value: String,
    operand: f32,
    operator: char,
    in_num: bool,
}

impl CalcState {
    


}
