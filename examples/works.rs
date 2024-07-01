use openalex::Work;

pub fn main() {
    let _work = Work::new("4251028522").unwrap();
    let _works_response = Work::get_samples(200, "1234").unwrap();
}
