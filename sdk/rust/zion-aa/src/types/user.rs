pub struct User {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: String,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub profile_picture: Option<String>,
}
