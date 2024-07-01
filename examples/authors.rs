use openalex::types::author::Author;

pub fn main() {
    let _author = Author::new("5010062957").unwrap();
    let _author_response = Author::get_samples(200, "1234").unwrap();
}
