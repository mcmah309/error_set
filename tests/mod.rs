

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
        let magazine_error = MagazineParsingError::MissingNameArg;
        let crate_error: SetLevelError = magazine_error.into();
        println!("{:?}", crate_error);
    
        let book_error = BookParsingError::MissingDescriptionArg;
        let crate_error_from_book: SetLevelError = book_error.into();
        println!("{:?}", crate_error_from_book);
    
        let x: Result<(), MagazineParsingError> = Err(MagazineParsingError::MissingNameArg);
        let _y: Result<(), BookParsingError> = x.map_err(Into::into);
    }
}