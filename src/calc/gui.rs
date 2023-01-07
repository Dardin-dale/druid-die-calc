use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};

#[derive(Clone, Data, Lens)]
pub struct CalcState {
    value: String, //result of die roll
    command: String,
    operand: String, //current operand
    operator: char,
    in_num: bool,
    ind_die: bool, //ensure only 1 'd' per die
}

impl CalcState {
    //add digit to display
    fn digit(&mut self, digit: u8) {
        if !self.in_num {
            self.operand.clear();
            self.in_num = true;
        }
        let ch = (b'0' + digit) as char;
        self.operand.push(ch);
    }

    fn display(&mut self) {}

    //Run from die roller
    fn compute(&mut self){}

    fn op(&mut self, op: char) {
        match op {
            '+' | '-' => {
                if !self.in_num {
                    self.command.push(self.operator);
                    self.operand.clear();
                }
                self.operator = op;
                self.in_num = false;
                self.command.push(op);
            }
            'c' => {
                self.operator.clear();
            }
            'C' => {
                self.command.clear();
            }
            'd' => {}
            '⌫' => {}
            _ => unreachable!(),
        }
    }
}
