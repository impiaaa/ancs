use nom::{error::ParseError, number::complete::le_u8, IResult};

/// Provides a set of identifiers for types of attributes that a consumer may require.
/// This list of `NotificationAttributeID`s follows the ANCS Specification for valid NotificationAttributeIDs
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NotificationAttributeID {
    AppIdentifier = 0,
    Title = 1,
    Subtitle = 2,
    Message = 3,
    MessageSize = 4,
    Date = 5,
    PositiveActionLabel = 6,
    NegativeActionLabel = 7,
}

impl From<NotificationAttributeID> for u8 {
    /// Convert a `NotificationAttributeID` to a `u8`:
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// let data: u8 = NotificationAttributeID::AppIdentifier.into();
    ///
    /// assert_eq!(0, data);
    /// ```
    fn from(original: NotificationAttributeID) -> u8 {
        match original {
            NotificationAttributeID::AppIdentifier => 0,
            NotificationAttributeID::Title => 1,
            NotificationAttributeID::Subtitle => 2,
            NotificationAttributeID::Message => 3,
            NotificationAttributeID::MessageSize => 4,
            NotificationAttributeID::Date => 5,
            NotificationAttributeID::PositiveActionLabel => 6,
            NotificationAttributeID::NegativeActionLabel => 7,
        }
    }
}

impl TryFrom<u8> for NotificationAttributeID {
    type Error = NotificationAttributeIDError;

    /// Attempts to convert a `u8` to a `NotificationAttributeID`
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// let attribute: NotificationAttributeID = NotificationAttributeID::try_from(0).unwrap();
    ///
    /// assert_eq!(NotificationAttributeID::AppIdentifier, attribute);
    /// ```
    ///
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(NotificationAttributeID::AppIdentifier),
            1 => Ok(NotificationAttributeID::Title),
            2 => Ok(NotificationAttributeID::Subtitle),
            3 => Ok(NotificationAttributeID::Message),
            4 => Ok(NotificationAttributeID::MessageSize),
            5 => Ok(NotificationAttributeID::Date),
            6 => Ok(NotificationAttributeID::PositiveActionLabel),
            7 => Ok(NotificationAttributeID::NegativeActionLabel),
            _ => Err(NotificationAttributeIDError),
        }
    }
}

impl NotificationAttributeID {
    /// Attempts to parse a `NotificationAttributeID` from a `&[u8]`
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// let data: [u8; 2] = [0, 1];
    /// let (data, notification_attribute_id) = NotificationAttributeID::parse(&data).unwrap();
    ///
    /// assert_eq!(NotificationAttributeID::AppIdentifier, notification_attribute_id);
    /// ```
    ///
    pub fn parse(i: &[u8]) -> IResult<&[u8], NotificationAttributeID> {
        let (i, notification_attribute_id) = le_u8(i)?;

        match NotificationAttributeID::try_from(notification_attribute_id) {
            Ok(notification_attribute_id) => Ok((i, notification_attribute_id)),
            Err(_) => Err(nom::Err::Failure(ParseError::from_error_kind(
                i,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }

    /// Determines if a `NotificationAttributeID` has a size based
    /// on requirements defined in the ANCS specification.
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// let app_identifier = NotificationAttributeID::AppIdentifier;
    /// let title = NotificationAttributeID::Title;
    ///
    /// assert_eq!(NotificationAttributeID::is_sized(app_identifier), false);
    /// assert_eq!(NotificationAttributeID::is_sized(title), true);
    /// ```
    ///
    pub fn is_sized(id: NotificationAttributeID) -> bool {
        match id {
            NotificationAttributeID::Title => true,
            NotificationAttributeID::Subtitle => true,
            NotificationAttributeID::Message => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotificationAttributeIDError;
