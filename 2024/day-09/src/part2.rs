#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (mut files, mut frees, _) = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .unwrap_or_else(|| panic!("'{}' should be digit", ch))
        })
        .enumerate()
        .fold(
            (Vec::new(), Vec::new(), 0),
            |(mut files, mut frees, block_index), (i, block_count)| {
                if i % 2 == 0 { &mut files } else { &mut frees }
                    .push(block_index..block_index + block_count);
                (files, frees, block_index + block_count)
            },
        );

    for file in files.iter_mut().rev() {
        for free in frees.iter_mut() {
            if free.start > file.start {
                break;
            }

            if free.len() >= file.len() {
                *file = free.start..free.start + file.len() as u32;
                free.start += file.len() as u32;
                break;
            }
        }
    }

    let checksum = files
        .iter()
        .cloned()
        .enumerate()
        .map(|(id, file_range)| id * file_range.sum::<u32>() as usize)
        .sum();

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "2333133121414131402";
        assert_eq!(2858, process(input)?);
        Ok(())
    }
}
