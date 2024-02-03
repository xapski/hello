enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    // secret file: admins only
    let access_level = Access::Guest;
    // determine if access level is authorized
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
}
