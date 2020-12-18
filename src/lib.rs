//! Traits for assembling and disassembling items.
use fehler::throws;

/// Describes a failure to assemble a composite from a sequence of parts.
#[derive(Debug, PartialEq)]
pub enum AssembleFailure<E> {
    /// There were not enough parts.
    Incomplete,
    /// There was an error with the parts.
    Error(E),
}

/// Converts an `E` into an `AssembleFailure`.
impl<E> From<E> for AssembleFailure<E> {
    #[inline]
    fn from(error: E) -> Self {
        Self::Error(error)
    }
}

/// Specifies the assembly of `Self` from a sequence of `P`s.
pub trait AssembleFrom<P>
where
    Self: Sized,
{
    /// Describes an error with the parts.
    ///
    /// This SHALL describe all cases that prevent a successful assembly other than the case where more parts are needed.
    type Error;

    /// Assembles `parts` into a `Self`.
    ///
    /// If assembly is successful, all items used in the assembly SHALL be removed from `parts`.
    ///
    /// # Errors
    ///
    /// If assembly fails, the cause of the failure SHALL be thrown and `parts` SHALL NOT be modified.
    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_from(parts: &mut Vec<P>) -> Self;
}

/// Specifies the assembly of `C` from a sequence of `Self`s.
pub trait AssembleInto<C>
where
    Self: Sized,
{
    /// Describes an error with the parts.
    ///
    /// This SHALL describe all cases that prevent a successful assembly other than the case where more parts are needed.
    type Error;

    /// Assembles `parts` into a `C`.
    ///
    /// If assembly is successful, all items used in the assembly SHALL be removed from `parts`.
    ///
    /// # Errors
    ///
    /// If assembly fails, the cause of the failure SHALL be thrown and `parts` SHALL NOT be modified.
    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_into(parts: &mut Vec<Self>) -> C;
}

/// For every `C` that implements `AssembleFrom<P>`, `P` SHALL implement `AssembleInto<S>`.
impl<P, C> AssembleInto<C> for P
where
    C: AssembleFrom<P>,
{
    type Error = <C as AssembleFrom<Self>>::Error;

    #[inline]
    #[throws(AssembleFailure<Self::Error>)]
    fn assemble_into(parts: &mut Vec<Self>) -> C {
        C::assemble_from(parts)?
    }
}

/// Disassembles a `C` into a sequence of `Self`s.
pub trait DisassembleFrom<C>
where
    Self: Sized,
{
    /// Describes an error that prevents disassembly.
    type Error;

    /// Disassembles `composite` into a sequence of `Self`s.
    ///
    /// # Errors
    ///
    /// If disassembly fails, the cause of the failure SHALL be thrown.
    #[throws(Self::Error)]
    fn disassemble_from(composite: C) -> Vec<Self>;
}

/// Disassembles `Self` into a sequence of `P`s.
pub trait DisassembleInto<P> {
    /// Describes an error that prevents disassembly.
    type Error;

    /// Disassembles `self` into a sequence of parts.
    ///
    /// # Errors
    ///
    /// If disassembly fails, the cause of the failure SHALL be thrown.
    #[throws(Self::Error)]
    fn disassemble_into(self) -> Vec<P>;
}

/// For every `P` that implements `DisassembleFrom<C>`, `C` SHALL implement `DisassembleInto<P>`.
impl<P, C> DisassembleInto<P> for C
where
    P: DisassembleFrom<C>,
{
    type Error = <P as DisassembleFrom<Self>>::Error;

    #[inline]
    #[throws(Self::Error)]
    fn disassemble_into(self) -> Vec<P> {
        P::disassemble_from(self)?
    }
}
