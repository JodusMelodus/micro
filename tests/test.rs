use micro::FromDTO;

#[test]
fn basic_conversion() {
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

#[test]
fn conversion_with_vec() {
    struct Test1 {
        names: Vec<String>,
        age: u8,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Test1)]
    struct Test2 {
        names: Vec<String>,
        age: u16,
    }

    let t1 = Test1 {
        names: vec!["John".to_string(), "Doe".to_string()],
        age: 23,
    };

    let t2 = Test2 {
        names: vec!["John".to_string(), "Doe".to_string()],
        age: 23,
    };

    let t: Test2 = t1.into();

    assert_eq!(t2, t);
}

#[test]
fn conversion_with_primitive_vec() {
    struct Test1 {
        names: String,
        age: Vec<u8>,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Test1)]
    struct Test2 {
        names: String,
        age: Vec<u8>,
    }

    let t1 = Test1 {
        names: "John".to_string(),
        age: vec![23, 83],
    };

    let t2 = Test2 {
        names: "John".to_string(),
        age: vec![23, 83],
    };

    let t: Test2 = t1.into();

    assert_eq!(t2, t);
}
