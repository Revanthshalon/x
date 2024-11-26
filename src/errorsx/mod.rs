use std::{backtrace::Backtrace, error::Error, fmt::Display, panic::Location};

#[derive(Debug)]
pub struct EnrichedErrors {
    message: String,
    backtrace: Backtrace,
    location: &'static Location<'static>,
    context: Vec<String>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl Display for EnrichedErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Context: {:?}\nSource:\n {:#?}",
            self.context, self.backtrace
        )
    }
}

impl Error for EnrichedErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

#[derive(Debug)]
pub struct EnrichedErrorsBuilder {
    message: String,
    context: Vec<String>,
    location: &'static Location<'static>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl EnrichedErrorsBuilder {
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::new(),
            location: Location::caller(),
            source: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn build(self) -> EnrichedErrors {
        EnrichedErrors {
            message: self.message,
            context: self.context,
            location: self.location,
            backtrace: Backtrace::force_capture(),
            source: self.source,
        }
    }
}

impl EnrichedErrors {
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        EnrichedErrorsBuilder::new(message).build()
    }

    #[track_caller]
    pub fn builder(message: impl Into<String>) -> EnrichedErrorsBuilder {
        EnrichedErrorsBuilder::new(message)
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn context(&self) -> &Vec<String> {
        &self.context
    }

    pub fn location(&self) -> &Location<'static> {
        self.location
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}
