mod common;

use {
    common::{Component, Item},
    conventus::AssembleFailure,
};

mod assemble_from {
    use {super::*, common::InvalidComponent, conventus::AssembleFrom};

    #[test]
    fn success() {
        let mut components = vec![Component(1), Component(2), Component(3)];
        assert_eq!(
            Item::assemble_from(&mut components).unwrap(),
            Item {
                first: Component(1),
                second: Component(2)
            }
        );
    }

    #[test]
    fn incomplete() {
        let mut components = vec![Component(1)];
        assert_eq!(
            Item::assemble_from(&mut components).unwrap_err(),
            AssembleFailure::Incomplete
        );
    }

    #[test]
    fn error() {
        let mut components = vec![Component(0), Component(1)];
        assert_eq!(
            Item::assemble_from(&mut components).unwrap_err(),
            AssembleFailure::Error(InvalidComponent)
        );
    }
}

mod assemble_into {
    use {super::*, common::InvalidComponent, conventus::AssembleInto};

    #[test]
    fn success() {
        let mut components = vec![Component(1), Component(2), Component(3)];
        let item: Item = Component::assemble_into(&mut components).unwrap();
        assert_eq!(
            item,
            Item {
                first: Component(1),
                second: Component(2)
            }
        );
    }

    #[test]
    fn incomplete() {
        let mut components = vec![Component(1)];
        let result: Result<Item, AssembleFailure<InvalidComponent>> =
            Component::assemble_into(&mut components);
        assert_eq!(result.unwrap_err(), AssembleFailure::Incomplete);
    }

    #[test]
    fn error() {
        let mut components = vec![Component(0), Component(1)];
        let result: Result<Item, AssembleFailure<InvalidComponent>> =
            Component::assemble_into(&mut components);
        assert_eq!(
            result.unwrap_err(),
            AssembleFailure::Error(InvalidComponent)
        );
    }
}

mod disassemble_from {
    use {super::*, common::InvalidItem, conventus::DisassembleFrom};

    #[test]
    fn success() {
        let item = Item {
            first: Component(1),
            second: Component(2),
        };
        assert_eq!(
            Component::disassemble_from(item).unwrap(),
            vec![Component(1), Component(2)]
        );
    }

    #[test]
    fn error() {
        let item = Item {
            first: Component(255),
            second: Component(2),
        };
        let result = Component::disassemble_from(item);
        assert_eq!(result.unwrap_err(), InvalidItem);
    }
}

mod disassemble_into {
    use {super::*, common::InvalidItem, conventus::DisassembleInto};

    #[test]
    fn success() {
        let item = Item {
            first: Component(1),
            second: Component(2),
        };
        let result: Result<Vec<Component>, InvalidItem> = item.disassemble_into();
        assert_eq!(result.unwrap(), vec![Component(1), Component(2)]);
    }

    #[test]
    fn error() {
        let item = Item {
            first: Component(255),
            second: Component(2),
        };
        let result: Result<Vec<Component>, InvalidItem> = item.disassemble_into();
        assert_eq!(result.unwrap_err(), InvalidItem);
    }
}
