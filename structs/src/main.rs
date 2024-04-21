struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// named tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct UnitLikeStruct;

fn main() {
    let mut user1 = User {
        email: String::from("someusername123"),
        username: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _unit_like_struct = UnitLikeStruct;
}

fn _build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
