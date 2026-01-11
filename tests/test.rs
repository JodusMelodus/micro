use micro::FromDTO;

#[test]
fn test1() {
    struct Test1 {
        name: String,
        age: u8,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Test1)]
    struct Test2 {
        name: String,
        age: u16,
    }

    let t1 = Test1 {
        name: "John".to_string(),
        age: 5,
    };

    let t2 = Test2 {
        name: "John".to_string(),
        age: 5,
    };

    let t: Test2 = t1.into();

    assert_eq!(t2, t);
}
