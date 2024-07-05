pub struct Filter {
    inner: String,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Filter {
    pub fn builder() -> FilterBuilder {
        FilterBuilder::default()
    }
}

#[derive(Default)]
pub struct FilterBuilder {
    inner: String,
}

impl FilterBuilder {
    pub fn new(mut self, key: &str, value: &str) -> FilterBuilder {
        self.inner.push_str(key);
        self.inner.push(':');
        self.inner.push_str(value);
        self
    }

    pub fn and(mut self, key: &str, value: &str) -> FilterBuilder {
        self.inner.push(',');
        self.inner.push_str(key);
        self.inner.push(':');
        self.inner.push_str(value);
        self
    }

    pub fn or(mut self, key: &str, value: &str) -> FilterBuilder {
        self.inner.push('|');
        self.inner.push_str(key);
        self.inner.push(':');
        self.inner.push_str(value);
        self
    }

    pub fn build(self) -> Filter {
        Filter { inner: self.inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter1() {
        let filter = Filter::builder()
            .new("institutions.country_code", "fr")
            .and("institutions.country_code", "gb")
            .or("institutions.country_code", "de")
            .build();

        let correct_filter = "institutions.country_code:fr,institutions.country_code:gb|institutions.country_code:de";

        assert_eq!(filter.to_string().as_str(), correct_filter);
    }
}
