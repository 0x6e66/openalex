pub struct Sort {
    inner: String,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Sort {
    pub fn builder() -> SortBuilder {
        SortBuilder::default()
    }
}

#[derive(Default)]
pub struct SortBuilder {
    inner: Vec<String>,
}

impl SortBuilder {
    pub fn add_sort(mut self, key: &str, direction: &str) -> SortBuilder {
        self.inner.push(format!("{key}:{direction}"));
        self
    }

    pub fn build(self) -> Sort {
        Sort {
            inner: self.inner.join(","),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort1() {
        let sort = Sort::builder()
            .add_sort("publication_year", "desc")
            .add_sort("relevance_score", "asc")
            .build();

        let correct_sort = "publication_year:desc,relevance_score:asc";

        assert_eq!(sort.to_string().as_str(), correct_sort);
    }
}
