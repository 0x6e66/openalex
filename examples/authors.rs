use openalex::types::{author::Author, filter::Filter, sort::Sort, APIEntity};

pub fn main() {
    let _author = Author::new("A5010062957").unwrap();
    let _author_response = Author::get_samples(20, "1234").unwrap();
    let _author_response = Author::filter(
        Filter::builder()
            .new("cited_by_count", ">30")
            .new("cited_by_count", "<100")
            .build(),
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
    let _author_response = Author::search(
        "machine learning",
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
}
