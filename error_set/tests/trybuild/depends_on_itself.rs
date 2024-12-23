use error_set::error_set;

error_set! {
    BookParsingError = {
        MissingDescriptionArg
    } || BookParsingError;
    BookSectionParsingError = {
        MissingNameArg,
        NoContents,
    };
}