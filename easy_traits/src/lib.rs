#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        self.to_owned()
    }
    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.value.push_str(&nb_to_append.to_string());
        self.to_owned()
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        self.value = self
            .value
            .chars()
            .filter(|cha| (*cha == '-' || !cha.is_ascii_punctuation()))
            .collect::<String>();
        self.to_owned()
    }
}
