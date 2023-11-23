use crate::common::{BarComposed, FooComponent};

pub struct BarComposedWithTypeParameter<TFoo: FooComponent> {
    foo: TFoo,
}

impl<TFoo: FooComponent> BarComposed for BarComposedWithTypeParameter<TFoo> {
    fn get_string(&self) -> String {
        self.foo.get_number().to_string()
    }
}

impl<TFoo: FooComponent> BarComposedWithTypeParameter<TFoo> {
    pub fn new(foo: TFoo) -> Self {
        Self { foo }
    }
}

#[cfg(test)]
mod test {
    use crate::common::{BarComposed, OneFooComponentImpl};
    use crate::common::{MockFooComponent, OtherFooComponentImpl};

    use super::BarComposedWithTypeParameter;

    #[test]
    fn composition_using_pointers_to_trait_objects() {
        let foo = OneFooComponentImpl {};

        let bar = BarComposedWithTypeParameter::new(foo);

        assert_eq!(bar.get_string(), "42")
    }

    #[test]
    fn switching_implementation_using_pointers_to_trait_objects() {
        let foo = OtherFooComponentImpl {};

        let bar = BarComposedWithTypeParameter::new(foo);

        assert_eq!(bar.get_string(), "21")
    }

    #[test]
    fn assertions_against_mock_using_pointers_to_trait_objects() {
        let mut mock = MockFooComponent::new();
        mock.expect_get_number().times(1).return_const(55u32);

        let bar = BarComposedWithTypeParameter { foo: mock };

        assert_eq!(bar.get_string(), "55")
    }
}
