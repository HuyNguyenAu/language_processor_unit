use std::{error::Error, fmt};

#[derive(Debug)]
pub struct BaseException {
    pub message: String,
    pub inner_exception: Option<Box<BaseException>>,
}

impl fmt::Display for BaseException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl Error for BaseException {}

impl BaseException {
    pub fn new(message: String, inner_exception: Option<Box<BaseException>>) -> Self {
        BaseException {
            message,
            inner_exception,
        }
    }
}

impl From<std::io::Error> for BaseException {
    fn from(error: std::io::Error) -> Self {
        BaseException::new(format!("{:#?}", error), None)
    }
}

impl From<&dyn Error> for BaseException {
    fn from(error: &dyn Error) -> Self {
        BaseException::new(format!("{:#?}", error), None)
    }
}

impl From<String> for BaseException {
    fn from(message: String) -> Self {
        BaseException::new(message, None)
    }
}

impl From<Exception> for BaseException {
    fn from(exception: Exception) -> Self {
        match exception {
            Exception::StartUpException(exception) => {
                BaseException::new(exception.message, exception.inner_exception)
            }
            // OpenAI client exceptions.
            Exception::OpenAIChatCompletionException(exception) => {
                BaseException::new(exception.message, exception.inner_exception)
            }
            Exception::OpenAIEmbeddingsException(exception) => {
                BaseException::new(exception.message, exception.inner_exception)
            }
            // Language logic unit exceptions.
            Exception::LanguageLogicException(exception) => {
                BaseException::new(exception.message, exception.inner_exception)
            }
            // Control unit exceptions.
            Exception::DecoderException(exception) => {
                BaseException::new(exception.message, exception.inner_exception)
            }
            Exception::ExecutorException(exception) => {
                BaseException::new(exception.message, exception.inner_exception)
            }
        }
    }
}

#[derive(Debug)]
pub enum Exception {
    StartUpException(BaseException),
    // OpenAI client exceptions.
    OpenAIChatCompletionException(BaseException),
    OpenAIEmbeddingsException(BaseException),
    // Language logic unit exceptions.
    LanguageLogicException(BaseException),
    // Control unit exceptions.
    DecoderException(BaseException),
    ExecutorException(BaseException),
}

impl fmt::Display for Exception {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(formatter, "{:#?}", self),
        }
    }
}
