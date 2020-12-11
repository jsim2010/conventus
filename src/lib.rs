//! Traits for assembling and disassembling items.
use fehler::throws;

/// A failure to assemble a composite from a sequence of components.
#[derive(Debug, PartialEq)]
pub enum AssembleFailure<E> {
    /// There were not enough components.
    Incomplete,
    /// There was an error with the components.
    Error(E),
}

impl<E> From<E> for AssembleFailure<E> {
    #[inline]
    fn from(value: E) -> Self {
        Self::Error(value)
    }
}

/// Converts a sequence of components of type `N` into `Self`.
pub trait AssembleFrom<N>
where
    Self: Sized,
{
    /// The type of the error that could be thrown during assembly.
    type Error;

    /// Converts `components` into a `Self`.
    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_from(components: &mut Vec<N>) -> Self;
}

/// Converts a sequence of `Self`s into a composite of type `S`.
pub trait AssembleInto<S>
where
    Self: Sized,
{
    /// The type of the error that could be thrown during assembly.
    type Error;

    /// Converts `components` into a `S`.
    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_into(components: &mut Vec<Self>) -> S;
}

impl<N, S> AssembleInto<S> for N
where
    S: AssembleFrom<N>,
{
    type Error = <S as AssembleFrom<Self>>::Error;

    #[inline]
    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_into(components: &mut Vec<Self>) -> S {
        S::assemble_from(components)?
    }
}

/// Converts a composite of type `S` into a `Vec<Self>`.
pub trait DisassembleFrom<S>
where
    Self: Sized,
{
    /// The type of the error that could be thrown during disassembly.
    type Error;

    /// Converts `composite` into a `Vec<Self>`.
    #[throws(Self::Error)]
    fn disassemble_from(composite: S) -> Vec<Self>;
}

/// Converts `Self` into a `Vec<N>`.
pub trait DisassembleInto<N> {
    /// The type of the error that could be thrown during disassembly.
    type Error;

    /// Converts `self` into a `Vec<N>`.
    #[throws(Self::Error)]
    fn disassemble_into(self) -> Vec<N>;
}

impl<N, S> DisassembleInto<N> for S
where
    N: DisassembleFrom<S>,
{
    type Error = <N as DisassembleFrom<Self>>::Error;

    #[inline]
    #[throws(Self::Error)]
    fn disassemble_into(self) -> Vec<N> {
        N::disassemble_from(self)?
    }
}
