// Timestamp: 2019-01-06 20:00:00
#[derive(Debug, PartialEq)]

// Tuple struct
pub struct Student(pub u32, pub String, pub String);

// Tuple struct methods are just functions that take a reference to the tuple struct as their first argument.
pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    student.2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_functions() {
        let student = Student(123, "John".to_string(), "Doe".to_string());

        assert_eq!(id(&student), 123);
        assert_eq!(first_name(&student), "John");
        assert_eq!(last_name(&student), "Doe");
    }
}
