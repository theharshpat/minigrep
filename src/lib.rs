pub fn search(query: &str, content: &str) -> Vec<String> {
    let mut v = vec![];
    for line in content.lines() {
        v.push(line.to_string());
    }

    v
}