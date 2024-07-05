use openalex::{
    api_entities::APIEntity,
    utils::{filter::Filter, sort::Sort},
    Work,
};

pub fn main() {
    let _work = Work::new("W4251028522").unwrap();
    let _works_response = Work::get_samples(20, "1234").unwrap();
    let _works_response = Work::filter(
        Filter::builder()
            .new("institutions.country_code", "fr")
            .and("institutions.country_code", "gb")
            .build(),
        1,
        20,
        Sort::builder().add_sort("publication_year", "desc").build(),
    )
    .unwrap();
    let _works_response = Work::search(
        "machine learning",
        1,
        20,
        Sort::builder().add_sort("publication_year", "desc").build(),
    )
    .unwrap();
}
