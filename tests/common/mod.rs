use {
    conventus::{AssembleFailure, AssembleFrom, DisassembleFrom},
    fehler::{throw, throws},
};

#[derive(Debug, PartialEq)]
pub struct Component(pub u8);

#[derive(Debug, PartialEq)]
pub struct Item {
    pub first: Component,
    pub second: Component,
}

#[derive(Debug, PartialEq)]
pub struct InvalidComponent;

#[derive(Debug, PartialEq)]
pub struct InvalidItem;

impl AssembleFrom<Component> for Item {
    type Error = InvalidComponent;

    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_from(components: &mut Vec<Component>) -> Self {
        if components.len() >= 2 {
            if components[0].0 == 0 || components[1].0 == 0 {
                throw!(AssembleFailure::Error(InvalidComponent));
            }

            Self {
                first: components.remove(0),
                second: components.remove(0),
            }
        } else {
            throw!(AssembleFailure::Incomplete);
        }
    }
}

impl DisassembleFrom<Item> for Component {
    type Error = InvalidItem;

    #[throws(InvalidItem)]
    fn disassemble_from(item: Item) -> Vec<Self> {
        if item.first.0 == 255 {
            throw!(InvalidItem);
        }

        vec![item.first, item.second]
    }
}
