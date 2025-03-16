use std::cmp::Ordering;


struct Student {
    first_name : String,
    last_name : String,
    grade_percentage : u8
}


impl Student {
    fn compare_student(&self, other_student : Student) -> String {
        match self.grade_percentage.cmp(&other_student.grade_percentage){
            Ordering::Greater => format!("{} {} has the higher grade out of the two.", self.first_name, self.last_name),
            Ordering::Less => format!("{} {} has the higher grade out of the two.", other_student.first_name, other_student.last_name),
            Ordering::Equal => format!("Both of them have the same grades.")
        }
    }   
}


fn main() {
    let student_one = Student {
        first_name : String::from("Mel"),
        last_name : String::from("E."),
        grade_percentage : 95
    };

    let student_two = Student {
        first_name : String::from("Jay"),
        last_name : String::from("M."),
        grade_percentage : 90
    };

    println!("{}", student_one.compare_student(student_two));
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn student_one_greater_than_student_two() {
        let student_one = Student {
            first_name : String::from("Mel"),
            last_name : String::from("E."),
            grade_percentage : 95
        };

        let student_two = Student {
            first_name : String::from("Jay"),
            last_name : String::from("M."),
            grade_percentage : 90
        };

        assert_eq!(String::from("Mel E. has the higher grade out of the two."), student_one.compare_student(student_two))
    }

    #[test]
    fn student_one_less_than_student_two() {
        let student_one = Student {
            first_name : String::from("Mel"),
            last_name : String::from("E."),
            grade_percentage : 85
        };

        let student_two = Student {
            first_name : String::from("Jay"),
            last_name : String::from("M."),
            grade_percentage : 90
        };

        assert_eq!(String::from("Jay M. has the higher grade out of the two."), student_one.compare_student(student_two))
    }

    #[test]
    fn student_one_equal_student_two() {
        let student_one = Student {
            first_name : String::from("Mel"),
            last_name : String::from("E."),
            grade_percentage : 90
        };

        let student_two = Student {
            first_name : String::from("Jay"),
            last_name : String::from("M."),
            grade_percentage : 90
        };

        assert_eq!(String::from("Both of them have the same grades."), student_one.compare_student(student_two))
    }
}