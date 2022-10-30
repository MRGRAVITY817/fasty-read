use std::{fs, io};

/// Count the match counts of alphabets inside the imported file.
pub fn count_alpha(file_path: &str, alphabets: &[char]) -> Result<usize, io::Error> {
    let file_text = fs::read_to_string(file_path)?;
    let counts = file_text.matches(alphabets).count();
    Ok(counts)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    fn should_count_given_alphabets_from_a_text_file() {
        // Arrange
        let file_path = "very_easy.txt";
        let alphabets = vec!['a', 'c'];
        fs::write(
            &file_path,
            "Concurrency is awesome! Of course, GlueSQL is more awesome :)".to_string(),
        )
        .unwrap();
        // Act
        let result = count_alpha(&file_path, &alphabets);
        // Assert
        assert_eq!(5, result.unwrap());
        // remove generated txt file
        fs::remove_file(&file_path).unwrap();
    }
}
