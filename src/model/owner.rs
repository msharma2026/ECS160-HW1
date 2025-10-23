pub struct Owner {
    pub login: String,
    pub id: String,
    pub htmlUrl: String,
    pub site_admin: String,
}
impl Owner {
    pub fn new(login: String, id: String, htmlUrl: String, site_admin: String) -> Self {
        Self { login, id, htmlUrl, site_admin }
    }
}