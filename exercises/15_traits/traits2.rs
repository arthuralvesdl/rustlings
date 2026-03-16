trait AppendBar {
    fn append_bar(self) -> Self;
    fn insert_bar(self, _: &str) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }

    fn insert_bar(mut self, tag: &str) -> Self {
        self.insert(0, tag.to_string());
        self
    }
}

fn main() {
    // You can optionally experiment here.
    let mut tag = vec![String::from("tag1"), String::from("tag2")];
    tag = tag.insert_bar("my string");
    tag = tag.append_bar();

    println!("{:?}", tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
