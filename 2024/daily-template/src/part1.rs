#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        todo!("haven't built test yet");
        let input = "";
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
