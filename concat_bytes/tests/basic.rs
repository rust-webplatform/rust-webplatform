#![feature(plugin)]
#![plugin(concat_bytes)]

#[test]
fn test_strings() {
    let bytes = concat_bytes!("Hello", " world");
    assert_eq!(bytes, b"Hello world");
}

#[test]
fn test_literals() {
    let bytes = concat_bytes!("String ", 42, 2.5f64, 'e', b'\0', b"world");
    assert_eq!(bytes, b"String 422.5e\0world");
}

#[test]
fn test_bytes() {
    let bytes = concat_bytes!(b"Hello ", b"world", b'!');
    assert_eq!(bytes, b"Hello world!");
}

#[test]
fn test_c_string() {
    let bytes = concat_bytes!("KokaKiwi", b'\0');
    assert_eq!(bytes, b"KokaKiwi\0");
}
