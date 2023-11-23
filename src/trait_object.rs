use crate::common::BarComposed;

use super::common::FooComponent;

pub struct BarComposedWithTraitObject {
    foo: Box<dyn FooComponent>,
}

impl BarComposedWithTraitObject {
    pub fn new(foo: Box<dyn FooComponent>) -> Self {
        Self { foo }
    }
}

impl BarComposed for BarComposedWithTraitObject {
    fn get_string(&self) -> String {
        self.foo.get_number().to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::common::{BarComposed, OneFooComponentImpl};
    use crate::common::{MockFooComponent, OtherFooComponentImpl};

    use super::BarComposedWithTraitObject;

    #[test]
    fn composition_using_pointers_to_trait_objects() {
        let foo = Box::new(OneFooComponentImpl {});

        let bar = BarComposedWithTraitObject { foo };

        assert_eq!(bar.get_string(), "42")
    }

    #[test]
    fn switching_implementation_using_pointers_to_trait_objects() {
        let foo = Box::new(OtherFooComponentImpl {});

        let bar = BarComposedWithTraitObject { foo };

        assert_eq!(bar.get_string(), "21")
    }

    #[test]
    fn assertions_against_mock_using_pointers_to_trait_objects() {
        let mut foo = MockFooComponent::new();
        foo.expect_get_number().times(1).return_const(55u32);

        let bar = BarComposedWithTraitObject { foo: Box::new(foo) };

        assert_eq!(bar.get_string(), "55")
    }
}
