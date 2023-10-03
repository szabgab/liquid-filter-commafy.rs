use liquid_core::{
    Display_filter, Filter, FilterReflection, ParseFilter, Result, Runtime, Value, ValueView,
};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "commafy",
    description = "Add comma to a number after every 3 digits.",
    parsed(CommafyFilter)
)]
pub struct Commafy;

#[derive(Debug, Default, Display_filter)]
#[name = "commafy"]
pub struct CommafyFilter;

impl Filter for CommafyFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let text = input.to_kstr();

        let mut chars: Vec<_> = text.chars().collect();
        let len = text.len();
        for i in (3..text.len()).step_by(3) {
            chars.insert(len - i, ',');
        }
        let commafied: String = chars.iter().collect();

        Ok(Value::scalar(commafied.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_commafy() {
        let cases = vec![
            ("5678", "5,678"),
            ("567", "567"),
            ("123456789", "123,456,789"),
        ];

        for entry in cases {
            let text = "{{ text | commafy}}";
            let globals = liquid::object!({
                "text": entry.0,
            });
            let template = liquid::ParserBuilder::with_stdlib()
                .filter(Commafy)
                .build()
                .unwrap()
                .parse(text)
                .unwrap();
            let output = template.render(&globals).unwrap();
            assert_eq!(output, entry.1.to_string());
        }
    }
}
