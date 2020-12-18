use {
    conventus::{AssembleFailure, AssembleFrom, AssembleInto, DisassembleFrom, DisassembleInto},
    fehler::{throw, throws},
};

#[derive(Clone, Debug, PartialEq)]
struct Part(u8);

#[derive(Clone, Debug, PartialEq)]
struct Composite {
    first: Part,
    second: Part,
}

impl AssembleFrom<Part> for Composite {
    type Error = ();

    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_from(components: &mut Vec<Part>) -> Self {
        if components.len() >= 2 {
            Self {
                first: components.remove(0),
                second: components.remove(0),
            }
        } else {
            throw!(());
        }
    }
}

impl DisassembleFrom<Composite> for Part {
    type Error = ();

    #[throws(Self::Error)]
    fn disassemble_from(composite: Composite) -> Vec<Self> {
        vec![composite.first, composite.second]
    }
}

#[test]
fn assemble_into() {
    let mut parts = vec![Part(1), Part(2)];

    assert_eq!(
        Composite::assemble_from(&mut parts.clone()),
        Part::assemble_into(&mut parts)
    );
}

#[test]
fn disassemble_into() {
    let composite = Composite {
        first: Part(1),
        second: Part(2),
    };

    assert_eq!(
        Part::disassemble_from(composite.clone()),
        composite.disassemble_into()
    );
}
