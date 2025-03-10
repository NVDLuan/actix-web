use std::env;

const AUTH_COOKIE: String = env::var("AUTH_COOKIE").unwrap_or_default();
const REFRESH_COOKIE: String = env::var("REFRESH_COOKIE").unwrap_or_default();

