use std::{fs, io, sync::Arc, thread, time::Instant};

/// Custom result type alias
pub type FastyResult<T> = Result<T, io::Error>;

/// Count the match counts of alphabets inside the imported file.
pub fn count_alpha(file_path: &str, alphabets: &[char]) -> FastyResult<u128> {
    let file_text = fs::read_to_string(file_path)?;
    let counts = file_text.matches(alphabets).count();
    Ok(counts as u128)
}

#[derive(Debug, PartialEq, Eq)]
pub struct CountOutput {
    counts: u128,
    elapsed: u128,
}

/// Count alphabets on multiple files (sequentially)
pub fn count_multiple_seq(file_paths: &[String], alphabets: &[char]) -> FastyResult<CountOutput> {
    let now = Instant::now(); // Timer start!

    let count_results = file_paths
        .iter()
        .map(|file_path| count_alpha(&file_path, &alphabets))
        .collect::<FastyResult<Vec<_>>>();

    let counts: u128 = count_results?.into_iter().sum();
    let elapsed = now.elapsed().as_micros(); // Timer ends.

    Ok(CountOutput { counts, elapsed })
}

/// Count alphabets on multiple files (concurrently)
pub fn count_multiple_concurrent(
    file_paths: Vec<String>,
    alphabets: Vec<char>,
) -> FastyResult<CountOutput> {
    let now = Instant::now(); // Timer start!

    const N_THREADS: usize = 8;
    let paths_count = file_paths.iter().count();
    let chunk_size = if paths_count % N_THREADS > 0 {
        paths_count / N_THREADS + 1
    } else {
        paths_count / N_THREADS
    };
    let worklists: Vec<_> = file_paths
        .chunks(chunk_size)
        .map(|chunk| chunk.to_owned())
        .collect();

    // Fork: Spawn a thread to handle each chunk
    let counts = worklists
        .into_iter()
        .map(move |file_paths| {
            let alphabets = Arc::new(alphabets.clone());
            thread::spawn(move || -> FastyResult<u128> {
                let alphabets = alphabets.clone();
                let result = file_paths
                    .iter()
                    .map(|file_path| {
                        let file_text = fs::read_to_string(file_path)?;
                        let counts = file_text.matches(alphabets.as_slice()).count();
                        Ok(counts as u128)
                    })
                    .collect::<FastyResult<Vec<_>>>()?;
                Ok(result.iter().sum())
            })
        })
        .map(|handle| handle.join().unwrap())
        .collect::<FastyResult<Vec<_>>>()?
        .iter()
        .sum();

    let elapsed = now.elapsed().as_micros(); // Timer ends.

    Ok(CountOutput { counts, elapsed })
}

#[cfg(test)]
mod test {
    use super::*;
    use lipsum::lipsum_words;
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
        let result = count_alpha(&file_path, &alphabets).unwrap();
        // remove generated txt file
        fs::remove_file(&file_path).unwrap();
        // Assert
        assert_eq!(5, result);
    }

    #[test]
    fn should_count_on_multiple_files_sequentially() {
        // Arrange
        let file_paths: Vec<String> = (0..16).map(|num| format!("data/{num}.txt")).collect();
        let words = 100000;
        file_paths.iter().for_each(|file_path| {
            let content = lipsum_words(words);
            fs::write(file_path, content).unwrap();
        });
        let alphabets = vec!['a', 'c'];
        // Act
        let result = count_multiple_seq(&file_paths, &alphabets).unwrap();
        // remove generated txt files
        file_paths.iter().for_each(|file_path| {
            fs::remove_file(file_path).unwrap();
        });
        // Assert
        assert_eq!(
            CountOutput {
                counts: 1000,
                elapsed: 1000
            },
            result
        );
        assert!(matches!(result, CountOutput { .. }));
    }

    #[test]
    fn should_count_on_multiple_files_concurrently() {
        // Arrange
        let file_paths: Vec<String> = (0..16).map(|num| format!("data/{num}.txt")).collect();
        let words = 100000;
        file_paths.iter().for_each(|file_path| {
            let content = lipsum_words(words);
            fs::write(file_path, content).unwrap();
        });
        let alphabets = vec!['a', 'c'];
        // Act
        let result = count_multiple_concurrent(file_paths.clone(), alphabets).unwrap();
        // remove generated txt files
        file_paths.iter().for_each(|file_path| {
            fs::remove_file(file_path).unwrap();
        });
        // Assert
        assert_eq!(
            CountOutput {
                counts: 1000,
                elapsed: 1000
            },
            result
        );
        // assert!(matches!(result, CountOutput { .. }));
    }
}
