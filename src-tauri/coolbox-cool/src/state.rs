use crate::tasks::ExecutableState;

pub trait StateAble {
    fn current_state(&mut self) -> &mut ExecutableState;

    fn outputs(&mut self) -> &mut Vec<String>;

    fn errors(&mut self) -> &mut Vec<String>;
}
