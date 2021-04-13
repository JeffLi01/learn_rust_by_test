#[test]
fn test_read () {
    use std::io::Cursor;
    use byteorder::{BigEndian, ReadBytesExt};
    
    let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
    // Note that we use type parameters to indicate which kind of byte order
    // we want!
    assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
    assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
}

#[test]
fn test_write () {
    use byteorder::{LittleEndian, WriteBytesExt};

    let mut wtr = vec![];
    wtr.write_u16::<LittleEndian>(517).unwrap();
    wtr.write_u16::<LittleEndian>(768).unwrap();
    assert_eq!(wtr, vec![0x05, 0x02, 0x00, 0x03]);
}
