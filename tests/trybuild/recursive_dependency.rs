use error_set::error_set;

error_set! {
    MediaError = {
        IoError(std::io::Error)
        } || BookParsingError || DownloadError || UploadError;
    BookParsingError = {
        MissingDescriptionArg
    } || BookSectionParsingError;
    BookSectionParsingError = {
        MissingNameArg,
        NoContents,
    } || BookParsingError;
    DownloadError = {
        CouldNotConnect,
        OutOfMemory(std::io::Error),
    };
    UploadError = {
        NoConnection(std::io::Error),
    };
}