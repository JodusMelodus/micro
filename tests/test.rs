use micro::FromDto;

#[test]
fn basic_conversion() {
    struct UserDto {
        name: String,
        age: u8,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(UserDto)]
    struct User {
        name: String,
        age: u16,
    }

    let user_dto = UserDto {
        name: "John".to_string(),
        age: 5,
    };

    let user: User = user_dto.into();

    assert_eq!(
        user,
        User {
            name: "John".to_string(),
            age: 5
        }
    );
}

#[test]
fn conversion_with_vec() {
    struct UserDto {
        names: Vec<String>,
        age: u8,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(UserDto)]
    struct User {
        names: Vec<String>,
        age: u16,
    }

    let user_dto = UserDto {
        names: vec!["John".to_string(), "Doe".to_string()],
        age: 23,
    };

    let user: User = user_dto.into();

    assert_eq!(
        user,
        User {
            names: vec!["John".to_string(), "Doe".to_string()],
            age: 23
        }
    );
}

#[test]
fn conversion_with_primitive_vec() {
    struct UserDto {
        name: String,
        ages: Vec<u8>,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(UserDto)]
    struct User {
        name: String,
        ages: Vec<u8>,
    }

    let user_dto = UserDto {
        name: "John".to_string(),
        ages: vec![23, 83],
    };

    let user: User = user_dto.into();

    assert_eq!(
        user,
        User {
            name: "John".to_string(),
            ages: vec![23, 83]
        }
    );
}

#[test]
fn enum_conversion() {
    struct UserDto {
        name: String,
        age: u8,
    }

    enum UserDtos {
        Cat(UserDto),
    }

    let a1 = UserDtos::Cat(UserDto {
        name: "Benjamin".to_string(),
        age: 5,
    });

    #[derive(FromDto, PartialEq, Debug)]
    #[from(UserDto)]
    struct User {
        name: String,
        age: u8,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(UserDtos)]
    enum Users {
        Cat(User),
    }

    let users: Users = a1.into();

    assert_eq!(
        users,
        Users::Cat(User {
            name: "Benjamin".to_string(),
            age: 5,
        })
    );
}

#[test]
fn multiple() {
    struct UserDto1 {
        id: i32,
    }

    struct UserDto2 {
        id: i32,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(UserDto1)]
    #[from(UserDto2)]
    struct User {
        id: i32,
    }

    let user_dto_1 = UserDto1 { id: 1 };
    let user_dto_2 = UserDto2 { id: 2 };

    let user_1: User = user_dto_1.into();
    let user_2: User = user_dto_2.into();

    assert_eq!(user_1, User { id: 1 });
    assert_eq!(user_2, User { id: 2 });
}

#[test]
fn generic() {
    struct NumberDto<T> {
        num: T,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(NumberDto<T>)]
    struct Number<T> {
        num: T,
    }

    let number_dto = NumberDto { num: 5 };
    let number: Number<i32> = number_dto.into();

    assert_eq!(number, Number { num: 5 });
}

#[test]
fn option_vec() {
    struct PlaylistDto {
        tracks: Option<Vec<u8>>,
    }

    #[derive(FromDto, PartialEq, Debug)]
    #[from(PlaylistDto)]
    struct Playlist {
        tracks: Option<Vec<u16>>,
    }

    let playlist_dto = PlaylistDto {
        tracks: Some(vec![5]),
    };
    let playlist: Playlist = playlist_dto.into();

    assert_eq!(
        playlist,
        Playlist {
            tracks: Some(vec![5])
        }
    );
}
