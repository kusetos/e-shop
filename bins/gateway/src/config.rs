pub struct Config {
    pub catalog_url:  String,
    pub basket_url:   String,
    pub ordering_url: String,
    pub identity_url: String,
    pub jwt_secret:   String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            catalog_url:  var("CATALOG_SERVICE_URL"),
            basket_url:   var("BASKET_SERVICE_URL"),
            ordering_url: var("ORDERING_SERVICE_URL"),
            identity_url: var("IDENTITY_SERVICE_URL"),
            jwt_secret:   var("JWT_SECRET"),
        }
    }
}

fn var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}
