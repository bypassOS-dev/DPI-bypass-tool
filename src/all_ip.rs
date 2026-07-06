use std::fs;
fn lookup_known_ip(domain: &str) -> Option<String>{
    let content =fs::read_to_string("knows_ip.txt").ok()?;
    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let mut parts = line.splitn(2, "=");

        let found_domain = parts.next()?.trim();
        let found_ip = parts.next()?.trim();

        if domain == found_domain {
            return Some(found_ip.to_string())
        }else {
            println!("This domain is not exist!");
        }
    }
    None
}