use openalex::{
    api_entities::APIEntity,
    utils::{filter::Filter, sort::Sort},
    Keyword,
};

pub fn main() {
    let _keyword = Keyword::new("diagnosis").unwrap();
    let _keyword_response = Keyword::get_samples(20, "1234").unwrap();
    let _keyword_response = Keyword::filter(
        Filter::builder()
            .new("cited_by_count", ">30")
            .and("cited_by_count", "<100")
            .build(),
        1,
        20,
        Sort::builder().add_sort("works_count", "desc").build(),
    )
    .unwrap();
    let _keyword_response = Keyword::search(
        "space",
        1,
        20,
        Sort::builder().add_sort("works_count", "desc").build(),
    )
    .unwrap();
}
