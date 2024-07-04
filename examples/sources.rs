use openalex::types::{filter::Filter, sort::Sort, source::Source, APIEntity};

pub fn main() {
    let _source = Source::new("S137773608").unwrap();
    let _source_response = Source::get_samples(20, "1234").unwrap();
    let _source_response = Source::filter(
        Filter::builder()
            .add_filter("has_issn", "true")
            .and()
            .add_filter("continent", "asia")
            .build(),
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
    let _source_response = Source::search(
        "machine learning",
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
}
