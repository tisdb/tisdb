use tisdb::{AttributeValue, CisowskiEngine, MemoryStorage};

#[test]
fn test_metadata_attributes_lifecycle() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    // 1. Tworzenie węzła z zadeklarowaną klasą
    let nid = engine.create_node(vec!["Osoba".into(), "Mężczyzna".into()]);
    
    // 2. Pobranie i modyfikacja atrybutów (Update)
    let node = engine.get_node_mut(&nid).unwrap();
    assert_eq!(node.header.revision(), 1); // Wersja po utworzeniu

    node.header.set_attribute("Wiek", AttributeValue::Integer(35));
    node.header.set_attribute("Aktywny", AttributeValue::Boolean(true));

    assert_eq!(node.header.revision(), 3); // Wersja urosła dwukrotnie po dwóch update'ach

    // 3. Sprawdzenie, czy dane przetrwały i są prawidłowego typu
    let fetched_node = engine.get_node(&nid).unwrap();
    assert_eq!(fetched_node.header.class_path(), &["Osoba", "Mężczyzna"]);
    
    match fetched_node.header.get_attribute("Wiek") {
        Some(AttributeValue::Integer(val)) => assert_eq!(*val, 35),
        _ => panic!("Nieprawidłowy typ lub brak atrybutu Wiek"),
    }
}