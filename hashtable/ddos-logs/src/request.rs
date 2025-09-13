use core::fmt;
use std::{error::Error, net::IpAddr};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use queue_lib::{DynamicQueue as Queue, QueueTrait};

#[derive(Debug)]
pub enum RequestParseError {
    MissingArgument,
    InvalidIpAddress,
    InvalidDateTime,
    InvalidFormat,
}

impl Error for RequestParseError {}
impl fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

pub struct RequestValidator;
pub struct RequestParser;

impl RequestParser {
  fn parse(args: &[&str]) -> Result<Request, RequestParseError> {
    let ip_str = args.get(0).ok_or_else(|| RequestParseError::MissingArgument)?;
    let date_str = args.get(1).ok_or_else(|| RequestParseError::MissingArgument)?;
    let time_str = args.get(2).ok_or_else(|| RequestParseError::MissingArgument)?;
    let endpoint_str = args.get(3).ok_or_else(|| RequestParseError::MissingArgument)?;

    let mut timestamp_queue = Queue::new();

    timestamp_queue.enqueue(
      RequestParser::parse_datetime(date_str, time_str)?
    );

    Ok(Request {
      ip: RequestParser::parse_ip(ip_str)?,
      count: 1,
      timestamp_queue ,
      endpoint: endpoint_str.to_string(),
    })
  }
  pub fn parse_ip(ip_str: &str) -> Result<IpAddr, RequestParseError> {
    RequestValidator::validate_ip_address(ip_str)
  }
  pub fn parse_datetime(date_str: &str, time_str: &str) -> Result<NaiveDateTime, RequestParseError> {
    RequestValidator::validate_datetime(date_str, time_str)
  }

  fn parse_endpoint(endpoint_str: &str) -> Result<String, RequestParseError> {
    if endpoint_str.is_empty() {
      return Err(RequestParseError::InvalidFormat);
    }
    Ok(String::from(endpoint_str))
  }
}

impl RequestValidator {
  fn validate_ip_address(ip_str: &str) -> Result<IpAddr, RequestParseError> {
        ip_str.trim().parse::<IpAddr>()
            .map_err(|_| RequestParseError::InvalidIpAddress)
  }

  fn validate_datetime(date_str: &str, time_str: &str) -> Result<NaiveDateTime, RequestParseError> {
    if date_str.is_empty() || time_str.is_empty() {
      return Err(RequestParseError::InvalidDateTime);
    }

    let Some(time_only) = NaiveTime::parse_from_str(time_str, "%H:%M:%S").ok() else {
      return Err(RequestParseError::InvalidDateTime);
    };

    let Some(date_only) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok() else {
       return Err(RequestParseError::InvalidDateTime);
    };

    Ok(NaiveDateTime::new(date_only, time_only))
  }
}

#[derive(Debug)]
pub struct Request {
    pub ip: IpAddr,
    pub count: u32,
    pub timestamp_queue: Queue<NaiveDateTime>,
    pub endpoint: String,
}

impl Request {
    pub fn build(args: &[&str]) -> Result<Self, RequestParseError> {
      RequestParser::parse(args)
  }
}
