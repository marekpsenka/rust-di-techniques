use crate::common::{BarComposed, FooComponent, OneFooComponentImpl, OtherFooComponentImpl};

#[cfg(test)]
use crate::common::MockFooComponent;

pub enum FooComponentVariant {
    One(OneFooComponentImpl),
    Other(OtherFooComponentImpl),
    #[cfg(test)]
    Mock(MockFooComponent),
}

impl FooComponent for FooComponentVariant {
    fn get_number(&self) -> u32 {
        match self {
            FooComponentVariant::One(one) => one.get_number(),
            FooComponentVariant::Other(other) => other.get_number(),
            #[cfg(test)]
            FooComponentVariant::Mock(mock) => mock.get_number(),
        }
    }
}

pub struct BarComposedWithSumType {
    foo: FooComponentVariant,
}

impl BarComposedWithSumType {
    pub fn new(foo: FooComponentVariant) -> Self {
        Self { foo }
    }
}

impl BarComposed for BarComposedWithSumType {
    fn get_string(&self) -> String {
        self.foo.get_number().to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::common::{BarComposed, OneFooComponentImpl};
    use crate::common::{MockFooComponent, OtherFooComponentImpl};
    use crate::sum_type::FooComponentVariant;

    use super::BarComposedWithSumType;

    #[test]
    fn composition_using_pointers_to_trait_objects() {
        let foo = FooComponentVariant::One(OneFooComponentImpl {});

        let bar = BarComposedWithSumType { foo };

        assert_eq!(bar.get_string(), "42")
    }

    #[test]
    fn switching_implementation_using_pointers_to_trait_objects() {
        let foo = FooComponentVariant::Other(OtherFooComponentImpl {});

        let bar = BarComposedWithSumType { foo };

        assert_eq!(bar.get_string(), "21")
    }

    #[test]
    fn assertions_against_mock_using_pointers_to_trait_objects() {
        let mut mock = MockFooComponent::new();
        mock.expect_get_number().times(1).return_const(55u32);

        let bar = BarComposedWithSumType {
            foo: FooComponentVariant::Mock(mock),
        };

        assert_eq!(bar.get_string(), "55")
    }
}
