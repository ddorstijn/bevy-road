use opendrive::{core::OpenDrive, Interpolatable};

#[test]
fn parse_geometry() {
    let xml: String = std::fs::read_to_string("./tests/data/Ex_Line-Spiral-Arc.xodr").unwrap();
    let doc: OpenDrive = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(doc.header.rev_major, 1);
    assert_eq!(doc.header.rev_minor, 8);

    for road in doc.road.iter() {
        let t = road.interpolate(1.0);
        println!("t.x: {}", t.translation.x);
        assert!(t.translation.x == road.plan_view.geometry.get(1).unwrap().x);
    }

    println!("{:?}", doc);
}

#[test]
fn parse_elevation() {
    let xml: String =
        std::fs::read_to_string("./tests/data/Ex_Camber_Straight_Profile.xodr").unwrap();
    let doc: OpenDrive = quick_xml::de::from_str(&xml).unwrap();

    println!("{:?}", doc);
}
