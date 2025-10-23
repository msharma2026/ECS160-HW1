pub struct Owner {
    login: String,
    id: u64,
    htmlUrl: String,
    site_admin: bool,
}
impl Owner {
    pub fn new(login: String, id: u64, htmlUrl: String, site_admin: bool) -> Self {
        Self { login, id, htmlUrl, site_admin }
    }
}