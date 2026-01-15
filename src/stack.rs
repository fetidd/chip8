use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
pub struct Stack(VecDeque<u16>);

impl Stack {
    pub fn pop(&mut self) -> u16 {
        self.0.pop_back().unwrap_or(0) // TODO should this be an Option or is returning a 0 on empty stack alright?
    }

    pub fn push(&mut self, value: u16) {
        self.0.push_back(value);
    }
}
