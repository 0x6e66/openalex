use openalex::{
    api_entities::APIEntity,
    utils::{filter::Filter, sort::Sort},
    Publisher,
};

pub fn main() {
    let _publisher = Publisher::new("P4310320990").unwrap();
    let _publisher_response = Publisher::get_samples(20, "1234").unwrap();
    let _publisher_response = Publisher::filter(
        Filter::builder()
            .new("cited_by_count", ">30")
            .and("cited_by_count", "<100")
            .build(),
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
    let _publisher_response = Publisher::search(
        "nature",
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
}
