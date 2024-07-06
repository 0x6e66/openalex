use openalex::{
    api_entities::{funder::FunderResponse, APIEntity},
    utils::{filter::Filter, sort::Sort},
    Funder,
};

pub fn main() {
    let s = std::fs::read_to_string("samples.json").unwrap();
    let _ar = serde_json::from_str::<FunderResponse>(&s).unwrap();
    return;
    let _funder = Funder::new("F4320321001").unwrap();
    let _funder_response = Funder::get_samples(20, "1234").unwrap();
    let _funder_response = Funder::filter(
        Filter::builder()
            .new("cited_by_count", ">30")
            .and("cited_by_count", "<100")
            .build(),
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
    let _funder_response = Funder::search(
        "DLR",
        1,
        20,
        Sort::builder().add_sort("cited_by_count", "desc").build(),
    )
    .unwrap();
}
