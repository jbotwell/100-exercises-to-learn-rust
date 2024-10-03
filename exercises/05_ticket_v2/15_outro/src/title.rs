use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

#[derive(Debug, Error)]
pub enum TitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

impl TryFrom<String> for TicketTitle {
    type Error = TitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TicketTitle::try_from(value.as_str())
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(TitleError::Empty);
        } else if value.len() > 50 {
            return Err(TitleError::TooLong);
        }

        Ok(TicketTitle(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
