use openalex::{
    types::{filter::Filter, sort::Sort},
    Work,
};

pub fn main() {
    let _work = Work::new("4251028522").unwrap();
    let _works_response = Work::get_samples(20, "1234").unwrap();
    let _works_response = Work::filter(
        Filter::builder()
            .add_filter("institutions.country_code", "fr")
            .and()
            .add_filter("institutions.country_code", "gb")
            .build(),
        1,
        20,
        Sort::builder().add_sort("publication_year", "desc").build(),
    )
    .unwrap();
}
