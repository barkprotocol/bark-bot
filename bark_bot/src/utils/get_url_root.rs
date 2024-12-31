use url::Url;

pub fn get_url_root(url_str: &str) -> Option<String> {
    // Try to parse the given URL string into a Url object
    let url = Url::parse(url_str).ok()?;
    
    // Extract the scheme (e.g., "http", "https")
    let scheme = url.scheme().to_string();
    
    // Extract the host (e.g., "example.com")
    let host = url.host().unwrap().to_owned();
    
    // Return the scheme and host as a combined string like "http://example.com"
    Some(scheme + "://" + &host.to_string())
}
