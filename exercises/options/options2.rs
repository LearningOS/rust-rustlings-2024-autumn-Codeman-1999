// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// Define a trait that standardizes how to display the grade
pub trait GradeDisplay {
    fn display(&self) -> String;
}

// Implement GradeDisplay for f32 (numeric grades)
impl GradeDisplay for f32 {
    fn display(&self) -> String {
        format!("{}", self) // Simply format the float
    }
}

// Implement GradeDisplay for String (alphabetic grades)
impl GradeDisplay for String {
    fn display(&self) -> String {
        self.clone() // Just return the grade as is (clone for String)
    }
}

// Implement the ReportCard's print method for any type that implements GradeDisplay
impl<T: GradeDisplay> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.grade.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
