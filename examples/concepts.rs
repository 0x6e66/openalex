use openalex::{
    api_entities::APIEntity,
    utils::{filter::Filter, sort::Sort},
    Concept,
};

pub fn main() {
    let _concept = Concept::new("C41008148").unwrap();
    let _concept_response = Concept::get_samples(20, "1234").unwrap();
    let _concept_response = Concept::filter(
        Filter::builder()
            .new("cited_by_count", ">30")
            .and("cited_by_count", "<100")
            .build(),
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
    let _concept_response = Concept::search(
        "Computer science",
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
}
