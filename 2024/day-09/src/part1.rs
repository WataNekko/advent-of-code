use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut files = {
        let mut pair_iter = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .unwrap_or_else(|| panic!("'{}' should be digit", ch))
            })
            .tuples();

        let mut files = pair_iter.by_ref().collect_vec();
        files.extend(pair_iter.into_buffer().map(|used_count| (used_count, 0)));
        files
    };

    let mut checksum = 0;
    let mut block_index = 0;
    let mut last_id = files.len() - 1;

    'for_loop: for id in 0.. {
        let (file_count, mut free_count) = files[id];

        let prev_block_index = block_index;
        block_index += file_count as usize;
        checksum += id * (prev_block_index..block_index).sum::<usize>();

        if id == last_id {
            break;
        }

        while free_count > 0 {
            let (last_file_count, _) = &mut files[last_id];

            let min = std::cmp::min(free_count, *last_file_count);
            free_count -= min;
            *last_file_count -= min;

            let prev_block_index = block_index;
            block_index += min as usize;
            checksum += last_id * (prev_block_index..block_index).sum::<usize>();

            if *last_file_count == 0 {
                last_id -= 1;
                if id == last_id {
                    break 'for_loop;
                }
            }
        }
    }

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "2333133121414131402";
        assert_eq!(1928, process(input)?);
        Ok(())
    }
}
