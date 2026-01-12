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

#[test]
fn enum_conversion() {
    struct Cat1 {
        name: String,
        age: u8,
    }

    enum Animal1 {
        Cat(Cat1),
    }

    let a1 = Animal1::Cat(Cat1 {
        name: "Benjamin".to_string(),
        age: 5,
    });


    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Cat1)]
    struct Cat2 {
        name: String,
        age: u8,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Animal1)]
    enum Animal2 {
        Cat(Cat2),
    }

    let a2 = Animal2::Cat(Cat2 {
        name: "Benjamin".to_string(),
        age: 5,
    });

    let a: Animal2 = a1.into();

    assert_eq!(a2, a);
}

#[test]
fn multiple() {
    struct Dog1 {
        id: i32,
    }

    struct Dog2 {
        id: i32,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Dog1)]
    #[from(Dog2)]
    struct Dog {
        id: i32,
    }

    let d1 = Dog1 { id: 1 };
    let d2 = Dog2 { id: 2 };
    let real_dog1 = Dog { id: 1 };
    let real_dog2 = Dog { id: 2 };

    let dog1: Dog = d1.into();
    let dog2: Dog = d2.into();

    assert_eq!(dog1, real_dog1);
    assert_eq!(dog2, real_dog2);
}

#[test]
fn generic() {
    struct Gen1<T> {
        a: T,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(Gen1<T>)]
    struct Gen2<T> {
        a: T,
    }

    let g1 = Gen1 { a: 5 };
    let g2 = Gen2 { a: 5 };
    let g: Gen2<i32> = g1.into();

    assert_eq!(g2, g);
}

#[test]
fn option_vec() {
    struct A {
        a: Option<Vec<u8>>,
    }

    #[derive(FromDTO, PartialEq, Debug)]
    #[from(A)]
    struct B {
        a: Option<Vec<u16>>,
    }

    let a = A { a: Some(vec![5]) };
    let b = B { a: Some(vec![5]) };
    let c: B = a.into();

    assert_eq!(b, c);
}
