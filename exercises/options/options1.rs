// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute rustlings hint quiz3 or use the hint watch subcommand for a hint.


// Define a ReportCard struct with a generic grade field
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
