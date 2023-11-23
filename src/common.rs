#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
pub trait FooComponent {
    fn get_number(&self) -> u32;
}

pub struct OneFooComponentImpl {}

impl FooComponent for OneFooComponentImpl {
    fn get_number(&self) -> u32 {
        42
    }
}

pub struct OtherFooComponentImpl {}

impl FooComponent for OtherFooComponentImpl {
    fn get_number(&self) -> u32 {
        21
    }
}

pub trait BarComposed {
    fn get_string(&self) -> String;
}
