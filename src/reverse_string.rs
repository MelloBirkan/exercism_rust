pub fn reverse(s: &str) -> String {
    let reverso = s.chars().rev().collect::<String>();
    reverso.to_string()
}
