pub struct Filter {
    inner: String,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        self.inner.to_owned()
    }
}

impl Filter {
    pub fn builder() -> FilterBuilder {
        FilterBuilder::default()
    }
}

#[derive(Default, PartialEq)]
enum LogicalOperations {
    And,
    Or,
    #[default]
    None,
}

#[derive(Default)]
pub struct FilterBuilder {
    inner: String,
    last_logical_op: LogicalOperations,
}

impl FilterBuilder {
    pub fn add_filter(mut self, key: &str, value: &str) -> FilterBuilder {
        self.inner.push_str(key);
        self.inner.push(':');
        self.inner.push_str(value);
        self.last_logical_op = LogicalOperations::None;
        self
    }

    pub fn and(mut self) -> FilterBuilder {
        self.inner.push(',');
        self.last_logical_op = LogicalOperations::And;
        self
    }

    pub fn or(mut self) -> FilterBuilder {
        self.inner.push('|');
        self.last_logical_op = LogicalOperations::Or;
        self
    }

    pub fn build(mut self) -> Filter {
        if self.last_logical_op != LogicalOperations::None {
            self.inner.remove(self.inner.len() - 1);
        }

        Filter { inner: self.inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter1() {
        let filter = Filter::builder()
            .add_filter("institutions.country_code", "fr")
            .and()
            .add_filter("institutions.country_code", "gb")
            .or()
            .add_filter("institutions.country_code", "de")
            .build();

        let correct_filter = "institutions.country_code:fr,institutions.country_code:gb|institutions.country_code:de";

        assert_eq!(filter.to_string().as_str(), correct_filter);
    }
}
