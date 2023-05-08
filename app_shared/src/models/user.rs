pub struct User {
    username: String,
    password: String,
    email: String,
    date_created: String,
    settings: Settings,
}

pub struct Settings {
    email_notifications: bool,
    certification: bool,
}
