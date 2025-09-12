use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let dt = Utc::now();
        Self {
            form_values: (field_name, field_value),
            date: dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            let err = FormError::new("name", self.name.clone(), "Username is empty");
            return Err(err);
        }
        if self.password.chars().count() < 8 {
            let err = FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            );
            return Err(err);
        }
        let mut has_sympbol = false;
        let mut has_number = false;
        let mut has_alpha = false;
        for ch in self.password.chars() {
            if ch.is_numeric() {
                has_number = true
            }
            if ch.is_ascii_punctuation() {
                has_sympbol = true
            }
             if ch.is_alphabetic() {
                has_alpha = true
            }
        }
        if !has_number || !has_sympbol || !has_alpha{
            let err = FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            );
            return Err(err);
        }
        Ok(())
    }
}
