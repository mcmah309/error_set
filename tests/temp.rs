use error_set::error_set;

error_set! {
    SetLevelError = {
        MissingNameArg,
        MissingPublishTimeArg,
        MissingDescriptionArg,
    };
    MagazineParsingError = {
        MissingNameArg,
    };
    BookParsingError = {
        MissingDescriptionArg,
    } || BookSectionParsingError;
    BookSectionParsingError = {
        MissingName,
        NoContents,
    };
}

fn temp() -> Result<(), BookSectionParsingError> {
    let y: Result<(), BookParsingError> = Err(BookParsingError::MissingDescriptionArg);
    let z = coerce!(y => {
        Ok(_) => { return Ok(())},
        Err(BookParsingError::MissingDescriptionArg) => 2;
        Err(BookParsingError) => return Err(BookSectionParsingError) // on the last pattern
    });
    Ok(())
}