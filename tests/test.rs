use named::Named;

#[test]
fn test() {
    #[derive(Named)]
    struct TypeNameA;

    #[derive(Named)]
    #[named(snake_case)]
    struct TypeNameB;

    #[derive(Named)]
    #[named(lowercase)]
    struct TypeNameC;

    #[derive(Named)]
    #[named(name = "dd")]
    struct TypeNameD;

    assert_eq!(TypeNameA::name(), "TypeNameA");
    assert_eq!(TypeNameB::name(), "type_name_b");
    assert_eq!(TypeNameC::name(), "typenamec");
    assert_eq!(TypeNameD::name(), "dd");
}
