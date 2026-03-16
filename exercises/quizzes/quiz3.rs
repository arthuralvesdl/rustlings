// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

// TODO: Adjust the struct as described above.
use std::fmt::Display;

//IMPLEMENTAÇÃO PESSOAL EXTRA EXERCÍCIO

struct NumericGrade(f32);

impl NumericGrade {
    fn new(value: f32) -> Result<Self, String> {
        if (0.0..=10.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err("Numeric grade must be between 0 and 10".into())
        }
    }
}

impl Display for NumericGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum LetterGrade {
    APlus,
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    CPlus,
    C,
    CMinus,
    DPlus,
    D,
    DMinus,
    FPlus,
    F,
    FMinus,
}

impl Display for LetterGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::APlus => "A+",
            Self::A => "A",
            Self::AMinus => "A-",
            Self::BPlus => "B+",
            Self::B => "B",
            Self::BMinus => "B-",
            Self::CPlus => "C+",
            Self::C => "C",
            Self::CMinus => "C-",
            Self::DPlus => "D+",
            Self::D => "D",
            Self::DMinus => "D-",
            Self::FPlus => "F+",
            Self::F => "F",
            Self::FMinus => "F-",
        };
        write!(f, "{}", value)
    }
}

//IMPLEMENTAÇÃO PESSOAL EXTRA EXERCÍCIO

struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
impl<T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.

    //IMPLEMENTAÇÃO PESSOAL EXTRA EXERCÍCIO

    let numeric_grade = NumericGrade::new(10.0).unwrap();
    let alphabetical_grade = LetterGrade::A;

    let report_my_grade_numericaly = ReportCard {
        grade: numeric_grade,
        student_age: 13,
        student_name: "Arthur".to_string(),
    };
    let report_my_grade_alphabetic = ReportCard {
        grade: alphabetical_grade,
        student_age: 13,
        student_name: "Arthur".to_string(),
    };

    println!("{:?}", report_my_grade_numericaly.print());
    println!("{:?}", report_my_grade_alphabetic.print());

    //IMPLEMENTAÇÃO PESSOAL EXTRA EXERCÍCIO
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
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
