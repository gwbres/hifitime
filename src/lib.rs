// Part of hifitime.
// TODO: Add LICENSE.txt and README.md

//! # hifitime 0.0.1
//!
//! Precise date and time handling in Rust built on top of
//! [` std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html).
//! The Epoch used is 01 Jan 1900 at midnight, but that should not matter in day-to-day use of this
//! library. This time corresponds to the TAI Epoch.
//!
//! Features:
//!
//! * Leap seconds (as announced by the IETF on a yearly basis)
//! * Julian dates and Modified Julian dates
//! * UTC representation with ISO8601 formatting
//! * Time varying `TimeZone`s to represent very high speed reference frames
//!
//! Most (all?) examples are validated with external references, as detailed on a test-by-test
//! basis.
//!
//! *NOTE:* Each time computing library may decide when the extra leap second exists as explained
//! in the [IETF leap second reference](https://www.ietf.org/timezones/data/leap-seconds.list).
//! To ease computation, `hifitime` decides that second is the 60th of a UTC date, if such exists.
//! Note that this second exists at a different time than defined on NASA HEASARC. That tool is
//! used for validation of Julian dates. As an example of how this is handled, check the Julian
//! day computations for [2015-06-30 23:59:59](https://goo.gl/o3KXSR),
//! [2015-06-30 23:59:60](https://goo.gl/QyUyrC) and [2015-07-01 00:00:00](https://goo.gl/Y25hpn).
//!
//! Does not includes:
//!
//! * Dates only, or times only (i.e. handles only the combination of both)
//! * Custom formatting of date time objects (for now)
//!
//! ## Usage
//! **WARNING: NOT YET AVAILABLE ON CARGO**
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hifitime = "0.0.1"
//! ```
//!
//! And add the following to your crate root:
//!
//! ```rust
//! extern crate hifitime;
//! ```
//!
//! ### Examples:
//!
//! ```rust
//! use hifitime::TimeSystem;
//! use hifitime::utc::{Utc, TimeZone};
//! use hifitime::instant::Duration;
//! use hifitime::julian::ModifiedJulian;
//!
//! let santa = Utc::new(2017, 12, 25, 01, 02, 14, 0).expect("Xmas failed");
//!
//! assert_eq!(
//!     santa.as_instant() + Duration::new(3600, 0),
//!     Utc::new(2017, 12, 25, 02, 02, 14, 0)
//!         .expect("Xmas failed")
//!         .as_instant(),
//!     "Could not add one hour to Christmas"
//! );
//! assert_eq!(format!("{}", santa), "2017-12-25T01:02:14+00:00");
//! assert_eq!(
//!     ModifiedJulian::from_instant(santa.as_instant()).days,
//!     58112.043217592596
//! );
//! assert_eq!(
//!     ModifiedJulian::from_instant(santa.as_instant()).julian_days(),
//!     2458112.5432175924
//! );
//! ```
//!

pub mod instant;
pub mod julian;
pub mod utc;

use std::cmp::PartialOrd;
use instant::Instant;
use std::fmt;

/// A TimeSystem enabled the creation of system for measuring spans of time, such as UTC or Julian
/// days.
pub trait TimeSystem: PartialOrd {
    fn from_instant(Instant) -> Self;
    fn as_instant(self) -> Instant;
}

#[derive(Debug)]
pub enum Errors {
    /// Carry is returned when a provided function does not support time carry. For example,
    /// if a call to `Utc::new` receives 60 seconds and there are only 59 seconds in the provided
    /// date time then a Carry Error is returned as the Result.
    Carry,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::Carry => write!(f, "a carry error (e.g. 61 seconds)"),
        }
    }
}
