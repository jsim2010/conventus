# Assembly 

## AssembleFrom

There SHALL be a defined interface for a type to take a sequence of items, modify it as needed, and attempt to convert a partition of the sequence into an instance of the type.

success
: The interface SHALL provide the ability to indicate a successful assembly by returning the created item.

incomplete
: The interface SHALL provide the ability to throw an error that indicates the sequence is incomplete.

error
: The interface SHALL provide the ability to throw an error defined by the implementation.

## AssembleInto

There SHALL be a defined interface for a type to take a sequence of itself, modify it as needed, and attempt to convert a partition of the sequence into a new type. This interface SHALL be automatically implemented for any implementation of the `AssembleFrom` interface.

success
: The interface SHALL provide the ability to indicate a successful assembly by returning the created item.

incomplete
: The interface SHALL provide the ability to throw an error that indicates the sequence is incomplete.

error
: The interface SHALL provide the ability to throw an error defined by the imple

# Disassembly

## DisassembleFrom

There SHALL be a defined interface for a type to convert itself into a sequence of items.

success
: The interface SHALL provide the ability to indicate a successful disassembly by returning the created sequence.

error
: The interface SHALL provide the ability to throw an error defined by the implementation.

## DisassembleInto

There SHALL be a defined interface for a type to convert itself into a sequence of items. This interface SHALL be automatically implemented for any implementation of the `DisassembleFrom` interface.

success
: The interface SHALL provide the ability to indicate a successful disassembly by returning the created sequence.

error
: The interface SHALL provide the ability to throw an error defined by the imple
