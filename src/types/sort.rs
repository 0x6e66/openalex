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
    inner: String,
}

impl SortBuilder {
    pub fn add_sort(mut self, key: &str, direction: &str) -> SortBuilder {
        self.inner.push_str(key);
        self.inner.push(':');
        self.inner.push_str(direction);
        self.inner.push(',');
        self
    }

    pub fn build(mut self) -> Sort {
        if let Some(c) = self.inner.chars().last() {
            if c == ',' {
                self.inner.remove(self.inner.len() - 1);
            }
        }

        Sort { inner: self.inner }
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
