//! AWS Regions and helper functions.
//!
//! Mostly used for translating the Region enum to a string AWS accepts.
//!
//! For example: `UsEast1` to "us-east-1"

use std::error::Error;
use std::str::FromStr;
use std::fmt::{Display, Error as FmtError, Formatter};

/// An AWS region.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Region {
    ApNortheast1,
    ApSoutheast1,
    ApSoutheast2,
    EuCentral1,
    EuWest1,
    SaEast1,
    UsEast1,
    UsWest1,
    UsWest2,
    CnNorth1,
}

/// An error produced when attempting to convert a `str` into a `Region` fails.
#[derive(Debug,PartialEq)]
pub struct ParseRegionError {
    message: String,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let region_str = match *self {
            Region::ApNortheast1 => "ap-northeast-1",
            Region::ApSoutheast1 => "ap-southeast-1",
            Region::ApSoutheast2 => "ap-southeast-2",
            Region::EuCentral1 => "eu-central-1",
            Region::EuWest1 => "eu-west-1",
            Region::SaEast1 => "sa-east-1",
            Region::UsEast1 => "us-east-1",
            Region::UsWest1 => "us-west-1",
            Region::UsWest2 => "us-west-2",
            Region::CnNorth1 => "cn-north-1",
        };

        write!(f, "{}", region_str)
    }
}

impl FromStr for Region {
    type Err = ParseRegionError;

    fn from_str(s: &str) -> Result<Region, ParseRegionError> {
        match s {
            "ap-northeast-1" => Ok(Region::ApNortheast1),
            "ap-southeast-1" => Ok(Region::ApSoutheast1),
            "ap-southeast-2" => Ok(Region::ApSoutheast2),
            "eu-central-1" => Ok(Region::EuCentral1),
            "eu-west-1" => Ok(Region::EuWest1),
            "sa-east-1" => Ok(Region::SaEast1),
            "us-east-1" => Ok(Region::UsEast1),
            "us-west-1" => Ok(Region::UsWest1),
            "us-west-2" => Ok(Region::UsWest2),
            "cn-north-1" => Ok(Region::CnNorth1),
            s => Err(ParseRegionError::new(s))
        }
    }
}

impl ParseRegionError {
    pub fn new(input: &str) -> Self {
        ParseRegionError {
            message: format!("Not a valid AWS region: {}", input)
        }
    }
}

impl Error for ParseRegionError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for ParseRegionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            "foo".parse::<Region>().err().expect(
                "Parsing foo as a Region was not an error"
            ).to_string(),
            "Not a valid AWS region: foo".to_owned()
        );
        assert_eq!("ap-northeast-1".parse(), Ok(Region::ApNortheast1));
        assert_eq!("ap-southeast-1".parse(), Ok(Region::ApSoutheast1));
        assert_eq!("ap-southeast-2".parse(), Ok(Region::ApSoutheast2));
        assert_eq!("eu-central-1".parse(), Ok(Region::EuCentral1));
        assert_eq!("eu-west-1".parse(), Ok(Region::EuWest1));
        assert_eq!("sa-east-1".parse(), Ok(Region::SaEast1));
        assert_eq!("us-east-1".parse(), Ok(Region::UsEast1));
        assert_eq!("us-west-1".parse(), Ok(Region::UsWest1));
        assert_eq!("us-west-2".parse(), Ok(Region::UsWest2));
        assert_eq!("cn-north-1".parse(), Ok(Region::CnNorth1));
    }

    #[test]
    fn region_display() {
        assert_eq!(Region::ApNortheast1.to_string(), "ap-northeast-1".to_owned());
        assert_eq!(Region::ApSoutheast1.to_string(), "ap-southeast-1".to_owned());
        assert_eq!(Region::ApSoutheast2.to_string(), "ap-southeast-2".to_owned());
        assert_eq!(Region::EuCentral1.to_string(), "eu-central-1".to_owned());
        assert_eq!(Region::EuWest1.to_string(), "eu-west-1".to_owned());
        assert_eq!(Region::SaEast1.to_string(), "sa-east-1".to_owned());
        assert_eq!(Region::UsEast1.to_string(), "us-east-1".to_owned());
        assert_eq!(Region::UsWest1.to_string(), "us-west-1".to_owned());
        assert_eq!(Region::UsWest2.to_string(), "us-west-2".to_owned());
        assert_eq!(Region::CnNorth1.to_string(), "cn-north-1".to_owned());
    }
}
