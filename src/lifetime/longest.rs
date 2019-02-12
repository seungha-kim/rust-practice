#[allow(dead_code)]
// 반환되는 참조의 lifetime은 'x랑 y중 짧은 쪽' 안에 들어와야 한다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_workd() {
        let a = String::from("haha");
        let b = "hoho";
        let _l = longest(a.as_str(), b);
    }
}
