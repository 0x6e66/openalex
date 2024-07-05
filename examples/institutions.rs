use openalex::types::{filter::Filter, institution::Institution, sort::Sort, APIEntity};

pub fn main() {
    let _institution = Institution::new("I4210142603").unwrap();
    let _institution_response = Institution::get_samples(20, "1234").unwrap();
    let _institution_response = Institution::filter(
        Filter::builder()
            .new("cited_by_count", ">30")
            .and("cited_by_count", "<100")
            .build(),
        1,
        20,
        Sort::builder().add_sort("works_count", "desc").build(),
    )
    .unwrap();
    let _institution_response = Institution::search(
        "raumfahrt",
        1,
        20,
        Sort::builder().add_sort("works_count", "desc").build(),
    )
    .unwrap();
}
