// to_url is a library crate that provides a function to_url that converts a string to a URL-encoded string.
pub fn to_url(s: &str) -> String {
    s.replace(' ', "%20")
}
