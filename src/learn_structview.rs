use structview::{u32_le, View};

#[derive(Clone, Copy, View)]
#[repr(C)]
struct Animal {
    name: [u8; 4],
    number_of_heads: u8,
    number_of_legs: u32_le,
}

#[test]
fn test_read () {
    let data = [0x43, 0x61, 0x74, 0x00, 0x01, 0x04, 0x00, 0x00, 0x00];
    let animal = Animal::view(&data).unwrap();

    assert_eq!(animal.name, *b"Cat\x00");
    assert_eq!(animal.number_of_heads, 1);
    assert_eq!(animal.number_of_legs.to_int(), 4);
}

#[test]
fn test_write () {
    use byteorder::{LittleEndian, WriteBytesExt};

    let data = [0x43, 0x61, 0x74, 0x00, 0x01, 0x04, 0x00, 0x00, 0x00];
    let mut animal = *Animal::view(&data).unwrap();

    assert_eq!(animal.number_of_legs.to_int(), 4);
    let mut buffer = vec![];
    buffer.write_u32::<LittleEndian>(5).unwrap();
    animal.number_of_legs = *u32_le::view(&buffer).unwrap();
    assert_eq!(animal.number_of_legs.to_int(), 5);
}
