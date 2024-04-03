

#[cfg(test)]
pub mod tests {
    use error_set::error_set;

    error_set!( SetLevelError,{
        MagazineParsingError {
            MissingNameArg,
            MissingPublishTimeArg
        },
        BookParsingError {
            MissingNameArg,
            MissingPublishTimeArg,
            MissingDescriptionArg,
        },
    });

    #[test]
    fn test() {
        let x = Test(1);
    }
}