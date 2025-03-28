use std::fmt;

pub struct ReportCard<T: fmt::Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}
