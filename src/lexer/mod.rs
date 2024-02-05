pub mod state;

/// Checking if a given character is a whitespace character. Currently this
/// this only checks '\r' and '\t', however there is a more exhaustive list
/// in the Rust lexer, which I might be able to update this to base off of.
fn is_whitespace(chr: char) -> bool {
    matches!(chr, ' ' | '\r' | '\t')
}

/// Checking if a given character is valid for identifiers. Currently according
/// to the grammar, this is all letters lowercase and uppercase, numbers, and
/// underscores.
fn is_valid_id(chr: char) -> bool {
    chr.is_alphanumeric() || matches!(chr, '_')
}

/// Checking if a given character is valid for the *start* of identifiers.
/// Currently according to the grammar, this is all letters lowercase and
/// uppercase, and underscores. We do not allow for numbers to be the start
/// because that can cause issues with parsing of actual numbers.
fn is_valid_id_start(chr: char) -> bool {
    chr.is_alphabetic() || matches!(chr, '_')
}
