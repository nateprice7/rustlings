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
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print_num(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
    }
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.letter_grade())
    }
    pub fn letter_grade(&self) -> String {
        match self.grade {
            1.0..=1.5 => "A+".to_string(),
            1.6..=2.0 => "A".to_string(),
            2.1..=2.5 => "A-".to_string(),
            2.6..=3.0 => "B+".to_string(),
            3.1..=3.5 => "B".to_string(),
            3.6..=4.0 => "B-".to_string(),
            4.1..=4.5 => "C+".to_string(),
            4.6..=5.0 => "C".to_string(),
            5.1..=5.5 => "C-".to_string(),
            _ => "F-".to_string(),
        }
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
            report_card.print_num(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 1.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
