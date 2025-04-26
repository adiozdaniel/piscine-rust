// Define a public function `talking` that returns a response based on the input text
pub fn talking(text: &str) -> &str {
    let trimmed = text.trim(); // Trim whitespace from both ends of the input string

    // If the trimmed input is empty, respond with a prompt to say something
    if trimmed.is_empty() {
        return "Just say something!";
    }

    // Determine if the statement is yelling (all uppercase letters before the last char and contains at least one alphabetic character)
    let is_yelling = trimmed[..trimmed.len() - 1].chars().all(is_uppercase)
        && trimmed.chars().any(char::is_alphabetic);

    // Determine if the statement is a question by checking the last character
    let is_question = match trimmed.chars().last() {
        Some('?') => true,
        _ => false,
    };

    // Return responses based on the type of input
    if is_question && is_yelling {
        "Quiet, I am thinking!" // Yelling question
    } else if is_question {
        "Sure." // Normal question
    } else if is_yelling {
        "There is no need to yell, calm down!" // Yelling statement
    } else {
        "Interesting" // All other cases
    }
}

// Helper function to determine if a character is uppercase or non-alphabetic
pub fn is_uppercase(c: char) -> bool {
    !c.is_alphabetic() || c.is_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_yell() {
        // Test cases where input is yelling
        assert_eq!(talking("JUST DO IT!"), "There is no need to yell, calm down!");
        assert_eq!(talking("1, 2, 3 GO!"), "There is no need to yell, calm down!");
        assert_eq!(talking("I LOVE YELLING"), "There is no need to yell, calm down!");
        assert_eq!(talking("WJDAGSAF ASVF EVA VA"), "There is no need to yell, calm down!");
    }

    #[test]
    fn test_question() {
        // Test cases where input is a normal question
        assert_eq!(talking("Hello how are you?"), "Sure.");
        assert_eq!(talking("Are you going to be OK?"), "Sure.");
        assert_eq!(talking("7?"), "Sure.");
        assert_eq!(talking("Like 15?"), "Sure.");
    }

    #[test]
    fn test_question_yelling() {
        // Test cases where input is a yelling question
        assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I am thinking!");
        assert_eq!(talking("ARE YOU FINISHED?"), "Quiet, I am thinking!");
        assert_eq!(talking("WHAT DID I DO?"), "Quiet, I am thinking!");
        assert_eq!(talking("ARE YOU COMING?"), "Quiet, I am thinking!");
    }

    #[test]
    fn test_interesting() {
        // Test cases where input is neither yelling nor a question
        assert_eq!(talking("something"), "Interesting");
        assert_eq!(talking("Wow that's good!"), "Interesting");
        assert_eq!(talking("Run far"), "Interesting");
        assert_eq!(talking("1 2 3 go!"), "Interesting");
        assert_eq!(talking("This is not ? a question."), "Interesting");
    }

    #[test]
    fn test_empty() {
        // Test cases for empty or whitespace-only input
        assert_eq!(talking(""), "Just say something!");
        assert_eq!(talking("										"), "Just say something!");
        assert_eq!(talking("          "), "Just say something!");
    }
}
