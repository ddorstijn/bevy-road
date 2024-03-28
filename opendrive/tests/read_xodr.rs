use opendrive::core::OpenDrive;

#[test]
fn parse_geometry() {
    let xml: String = std::fs::read_to_string("./tests/data/Ex_Line-Spiral-Arc.xodr").unwrap();
    let doc: OpenDrive = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(doc.header.rev_major, 1);
    assert_eq!(doc.header.rev_minor, 8);

    println!("{:?}", doc);
}

#[test]
fn parse_elevation() {
    let xml: String =
        std::fs::read_to_string("./tests/data/Ex_Camber_Straight_Profile.xodr").unwrap();
    let doc: OpenDrive = quick_xml::de::from_str(&xml).unwrap();

    println!("{:?}", doc);
}
