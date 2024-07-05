use openalex::{
    api_entities::APIEntity,
    utils::{filter::Filter, sort::Sort},
    Topic,
};

pub fn main() {
    let _topic = Topic::new("W4251028522").unwrap();
    let _topics_response = Topic::get_samples(20, "1234").unwrap();
    let _topics_response = Topic::filter(
        Filter::builder()
            .new("institutions.country_code", "fr")
            .and("institutions.country_code", "gb")
            .build(),
        1,
        20,
        Sort::builder().add_sort("publication_year", "desc").build(),
    )
    .unwrap();
    let _topics_response = Topic::search(
        "machine learning",
        1,
        20,
        Sort::builder().add_sort("publication_year", "desc").build(),
    )
    .unwrap();
}
