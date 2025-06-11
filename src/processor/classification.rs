use regex::Regex;
use std::collections::HashSet;

/// Extracts a list of technologies from a job description.
/// This is a basic stub and can be expanded with more sophisticated matching.
pub fn extract_tech_stack(description: &str) -> Vec<String> {
    let mut found_tech = HashSet::new();
    // Example keywords - this list should be significantly expanded and refined
    let keywords = vec![
        "Rust",
        "Python",
        "Java",
        "JavaScript",
        "TypeScript",
        "Go",
        "C++",
        "C#",
        "React",
        "Angular",
        "Vue",
        "Node.js",
        "Django",
        "Flask",
        "Spring",
        "SQL",
        "NoSQL",
        "PostgreSQL",
        "MySQL",
        "MongoDB",
        "Redis",
        "AWS",
        "Azure",
        "GCP",
        "Docker",
        "Kubernetes",
        "Terraform",
        "Linux",
        "Git",
        "Agile",
        "Scrum",
    ];

    for keyword in keywords {
        // Case-insensitive search for whole words
        let re = Regex::new(&format!(r"(?i)(?:^|\W){}(?:\W|$)", regex::escape(keyword))).unwrap();
        if re.is_match(description) {
            found_tech.insert(keyword.to_string());
        }
    }

    found_tech.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple() {
        let description = "We are looking for a Rust developer with experience in Python and AWS.";
        let tech_stack = extract_tech_stack(description);
        assert!(tech_stack.contains(&"Rust".to_string()));
        assert!(tech_stack.contains(&"Python".to_string()));
        assert!(tech_stack.contains(&"AWS".to_string()));
        assert_eq!(tech_stack.len(), 3);
    }

    #[test]
    fn test_extract_case_insensitive() {
        let description = "Experience with rust and PYTHON is required.";
        let tech_stack = extract_tech_stack(description);
        assert!(tech_stack.contains(&"Rust".to_string()));
        assert!(tech_stack.contains(&"Python".to_string()));
        assert_eq!(tech_stack.len(), 2);
    }

    #[test]
    fn test_extract_no_matches() {
        let description = "Looking for a project manager.";
        let tech_stack = extract_tech_stack(description);
        assert!(tech_stack.is_empty());
    }

    #[test]
    fn test_extract_with_punctuation() {
        let description = "Skills: Java, Kubernetes. Nice to have: C++.";
        let tech_stack = extract_tech_stack(description);
        assert!(tech_stack.contains(&"Java".to_string()));
        assert!(tech_stack.contains(&"Kubernetes".to_string()));
        assert!(tech_stack.contains(&"C++".to_string()));
        assert_eq!(tech_stack.len(), 3);
    }
}
