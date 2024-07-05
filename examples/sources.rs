use openalex::{
    api_entities::APIEntity,
    utils::{filter::Filter, sort::Sort},
    Source,
};

pub fn main() {
    let _source = Source::new("S137773608").unwrap();
    let _source_response = Source::get_samples(20, "1234").unwrap();
    let _source_response = Source::filter(
        Filter::builder()
            .new("has_issn", "true")
            .and("continent", "asia")
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
