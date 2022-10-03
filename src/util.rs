/// Split a string containing many SQL queries seperated by ';' into individual queries.
pub fn split_queries(s: &str) -> impl Iterator<Item = &str> {
    s.split(";").map(|s| s.trim()).filter(|s| s.len() > 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split() {
        let foo = "hello; world;";
        assert_eq!(split_queries(foo).collect::<Vec<_>>(), vec!["hello", "world"]);
    }
}
