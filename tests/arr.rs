#[macro_use]
extern crate generic_array;
extern crate typenum;

#[test]
fn empty_without_trailing_comma() {
    let ar = arr![u8; ];
    assert_eq!(format!("{:x}", ar), "");
}

#[test]
fn empty_with_trailing_comma() {
    let ar = arr![u8; , ];
    assert_eq!(format!("{:x}", ar), "");
}

#[test]
fn without_trailing_comma() {
    let ar = arr![u8; 10, 20, 30];
    assert_eq!(format!("{:x}", ar), "0a141e");
}

#[test]
fn with_trailing_comma() {
    let ar = arr![u8; 10, 20, 30, ];
    assert_eq!(format!("{:x}", ar), "0a141e");
}

#[test]
fn const_context() {
    const AR: generic_array::GenericArray<u8, typenum::U3> = arr![u8; 10, 20, 30];
    assert_eq!(format!("{:x}", AR), "0a141e");
}
