#[macro_export]
macro_rules! is_upper {
    ($c: expr) => {
        $c >= 'A' && $c <= 'Z'
    };
}

#[macro_export]
macro_rules! is_lower {
    ($c: expr) => {
        $c >= 'a' && $c <= 'z'
    };
}

#[macro_export]
macro_rules! is_number {
    ($c: expr) => {
        $c >= '0' && $c <= '9'
    };
}

#[macro_export]
macro_rules! is_letter {
    ($c: expr) => {
        is_upper!($c) || is_lower!($c)
    };
}

#[macro_export]
macro_rules! is_whitespace {
    ($c: expr) => {
        $c == ' ' || $c == '\n' || $c == '\t' || $c == '\r'
    };
}

#[macro_export]
macro_rules! is_separator {
    ($c: expr) => {
        $c == '.' || $c == '•'
    };
}

#[macro_export]
macro_rules! is_vowel {
    ($c: expr) => {
        $c == 'a' || $c == 'e' || $c == 'i' || $c == 'o' || $c == 'u'
    };
}

#[macro_export]
/// converts a single char into a number
macro_rules! to_number {
    ($c: expr) => {
        ($c as u8) - ('0' as u8)
    };
}
