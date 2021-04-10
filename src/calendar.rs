use std::fmt::Formatter;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::{
    DateTime, Datelike, Duration, Local, Timelike, Weekday,
};
use lazy_static::lazy_static;
use regex::Regex;
use unwrap::unwrap;

#[derive(Debug)]
pub enum Timer {
    Timers(Vec<Timer>),
    Calendar(Calendar),
    Duration {
        kind: DurationKind,
        duration: InDuration,
    },
    In(In),
}

pub enum In {
    Sec(InSec),
    Min(InMin),
    Hour(InHour),
    Day(InDay),
    Week(InWeek),
    Month(InMonth),
}

impl Debug for In {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            In::Sec(InSec(i)) => write!(f, "in {}s", i),
            In::Min(InMin(i, None)) => write!(f, "in {}m", i),
            In::Min(InMin(i, Some(a))) => {
                write!(f, "in {}m at {}", i, a)
            }
            In::Hour(InHour(i, None)) => write!(f, "in {}h", i),
            In::Hour(InHour(i, Some(a))) => {
                write!(f, "in {}h at {}", i, a)
            }
            In::Day(InDay(i, None)) => write!(f, "in {}d", i),
            In::Day(InDay(i, Some(a))) => {
                write!(f, "in {}d at {}", i, a)
            }
            In::Week(InWeek(i, None)) => write!(f, "in {}w", i),
            In::Week(InWeek(i, Some(a))) => {
                write!(f, "in {}w at {}", i, a)
            }
            In::Month(InMonth(i, None)) => write!(f, "in {}M", i),
            In::Month(InMonth(i, Some(a))) => {
                write!(f, "in {}M at {}", i, a)
            }
        }
    }
}

pub trait TimerAble {
    fn timer(&self) -> String;
}

impl TimerAble for DateTime<Local> {
    fn timer(&self) -> String {
        format!("OnCalendar={}", self.format("%Y-%m-%d %H:%M:%S"))
    }
}

trait AtAble {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local>;
}

pub struct AtSec(u32);
impl AtAble for AtSec {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        date.with_second(self.0).unwrap()
    }
}
impl Display for AtSec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{:02}", self.0)
    }
}
impl FromStr for AtSec {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r":(?P<s>\d\d?)").unwrap();
        }
        let cap = RE.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid seconds specifier.",
            s
        )))?;
        //TODO check if valid number of seconds
        Ok(Self(
            u32::from_str(cap.name("s").unwrap().as_str()).unwrap(),
        ))
    }
}
pub struct InSec(i64);
impl TimerAble for InSec {
    fn timer(&self) -> String {
        (Local::now() + Duration::seconds(self.0)).timer()
    }
}
pub struct AtMin(u32, Option<AtSec>);
impl AtAble for AtMin {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        let date =
            date.with_minute(self.0).unwrap().with_second(0).unwrap();
        if let Some(at) = &self.1 {
            at.at(date)
        } else {
            date
        }
    }
}
impl Display for AtMin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AtMin(i, None) => write!(f, ":{:02}:00", i),
            AtMin(i, Some(s)) => write!(f, ":{:02}{}", i, s),
        }
    }
}
impl FromStr for AtMin {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r":(?P<m>\d\d?)(?P<s>:\d\d?)?").unwrap();
        }
        let cap = RE.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid minutes specifier.",
            s
        )))?;
        //TODO check if valid number of seconds
        Ok(Self(
            u32::from_str(cap.name("m").unwrap().as_str()).unwrap(),
            if cap.name("s").is_some() {
                Some(AtSec::from_str(
                    cap.name("s").unwrap().as_str(),
                )?)
            } else {
                None
            },
        ))
    }
}
pub struct InMin(i64, Option<AtSec>);
impl TimerAble for InMin {
    fn timer(&self) -> String {
        let now = Local::now() + Duration::minutes(self.0);
        (if let Some(at) = &self.1 {
            at.at(now)
        } else {
            now
        })
        .timer()
    }
}
pub struct AtHour(u32, Option<AtMin>);
impl AtAble for AtHour {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        let date = date
            .with_hour(self.0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap();
        if let Some(at) = &self.1 {
            at.at(date)
        } else {
            date
        }
    }
}
impl Display for AtHour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AtHour(i, None) => write!(f, "{:02}:00:00", i),
            AtHour(i, Some(a)) => write!(f, "{:02}{}", i, a),
        }
    }
}
impl FromStr for AtHour {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\b(?P<h>\d\d?)(?P<m>:\d\d?(:\d\d?)?)?")
                    .unwrap();
        }
        let cap = RE.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid hours specifier.",
            s
        )))?;
        //TODO check if valid number of seconds
        Ok(Self(
            u32::from_str(cap.name("h").unwrap().as_str()).unwrap(),
            if cap.name("m").is_some() {
                Some(AtMin::from_str(
                    cap.name("m").unwrap().as_str(),
                )?)
            } else {
                None
            },
        ))
    }
}
pub struct InHour(i64, Option<AtMin>);
impl TimerAble for InHour {
    fn timer(&self) -> String {
        let now = Local::now() + Duration::hours(self.0);
        (if let Some(at) = &self.1 {
            at.at(now)
        } else {
            now
        })
        .timer()
    }
}
pub struct AtDay(u32, Option<AtHour>);
impl AtAble for AtDay {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        let date = unwrap!(
            date.with_day(self.0),
            "The {}-{}-{} does not exist.",
            date.year(),
            date.month(),
            self.0
        )
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
        if let Some(at) = &self.1 {
            at.at(date)
        } else {
            date
        }
    }
}
impl Display for AtDay {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            AtDay(i, None) => write!(f, "{}. 00:00:00", i),
            AtDay(i, Some(a)) => write!(f, "{}. {}", i, a),
        }
    }
}
impl FromStr for AtDay {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<d>\d\d?).?\s*\b(?P<h>\d\d?(:\d\d?(:\d\d?)?)?)?"
            )
            .unwrap();
        }
        let cap = RE.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid day specifier.",
            s
        )))?;
        //TODO check if valid number of days
        Ok(Self(
            u32::from_str(cap.name("d").unwrap().as_str()).unwrap(),
            if cap.name("h").is_some() {
                Some(AtHour::from_str(
                    cap.name("h").unwrap().as_str(),
                )?)
            } else {
                None
            },
        ))
    }
}
pub struct InDay(i64, Option<AtHour>);
impl TimerAble for InDay {
    fn timer(&self) -> String {
        let now = Local::now() + Duration::days(self.0);
        (if let Some(at) = &self.1 {
            at.at(now)
        } else {
            now
        })
        .timer()
    }
}
pub struct AtWeekDay(Weekday, Option<AtHour>);
impl AtAble for AtWeekDay {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        let date = (date
            - Duration::days(
                date.weekday().number_from_monday().into(),
            )
            + Duration::days(self.0.number_from_monday().into()))
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
        if let Some(at) = &self.1 {
            at.at(date)
        } else {
            date
        }
    }
}
impl Display for AtWeekDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AtWeekDay(i, None) => write!(f, "{} 00:00:00", i),
            AtWeekDay(i, Some(a)) => write!(f, "{} {}", i, a),
        }
    }
}
impl FromStr for AtWeekDay {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<d>[A-Za-z]*)\s*(?P<h>\d\d?(:\d\d?(:\d\d?)?)?)?"
            )
            .unwrap();
        }
        let cap = RE.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid weekday specifier.",
            s
        )))?;
        //
        Ok(Self(
            Weekday::from_str(cap.name("d").unwrap().as_str())
                .or_else(|_| {
                    Err(ParseError(format!(
                        "`{}` is not a valid Weekday",
                        cap.name("d").unwrap().as_str()
                    )))
                })?,
            if cap.name("h").is_some() {
                Some(AtHour::from_str(
                    cap.name("h").unwrap().as_str(),
                )?)
            } else {
                None
            },
        ))
    }
}

pub struct InWeek(i64, Option<WeekSub>);
impl TimerAble for InWeek {
    fn timer(&self) -> String {
        let now = Local::now() + Duration::weeks(self.0);
        (if let Some(at) = &self.1 {
            at.at(now)
        } else {
            now
        })
        .timer()
    }
}

pub enum WeekSub {
    AtHour(AtHour),
    AtWeekDay(AtWeekDay),
}
impl AtAble for WeekSub {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        match self {
            Self::AtHour(at) => at.at(date),
            Self::AtWeekDay(at) => at.at(date),
        }
    }
}
impl Display for WeekSub {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WeekSub::AtHour(a) => write!(f, "{}", a),
            WeekSub::AtWeekDay(a) => write!(f, "{}", a),
        }
    }
}
impl FromStr for WeekSub {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let wd = AtWeekDay::from_str(s);
        if let Err(wde) = wd {
            let h = AtHour::from_str(s);
            if let Err(he) = h {
                Err(ParseError(format!(
                    "Could not parse `WeekSub` because: {}, {}",
                    wde.0, he.0
                )))
            } else {
                Ok(WeekSub::AtHour(h.unwrap()))
            }
        } else {
            Ok(WeekSub::AtWeekDay(wd.unwrap()))
        }
    }
}

pub struct AtNthWeekDay(i8, Weekday, Option<AtHour>);
impl AtAble for AtNthWeekDay {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        let wanted = if self.0 > 0 {
            let tmp = date.with_day(1).unwrap();
            (if tmp.weekday().number_from_monday()
                > self.1.number_from_monday()
            {
                self.1.number_from_monday() + 7
                    - tmp.weekday().number_from_monday()
            } else {
                self.1.number_from_monday()
                    - tmp.weekday().number_from_monday()
            }) + (7 * self.0) as u32
        } else {
            let tmp =
                date.with_day(get_days_from_month(date) - 1).unwrap();
            let wanted: i32 = (if tmp.weekday().number_from_monday()
                < self.1.number_from_monday()
            {
                self.1.number_from_monday()
                    - 7
                    - tmp.weekday().number_from_monday()
            } else {
                self.1.number_from_monday()
                    - tmp.weekday().number_from_monday()
            }) as i32
                + (7 * self.0) as i32
                + tmp.day() as i32;
            if wanted > 0 {
                wanted as u32
            } else {
                panic!(
                    "{}-{} does not have {} {}s",
                    date.year(),
                    date.month(),
                    -self.0,
                    self.1
                )
            }
        };
        let date = unwrap!(
            date.with_day(wanted),
            "{}-{} does not have {} {}s",
            date.year(),
            date.month(),
            self.0,
            self.1
        )
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();
        if let Some(at) = &self.2 {
            at.at(date)
        } else {
            date
        }
    }
}
impl Display for AtNthWeekDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AtNthWeekDay(n, d, None) => {
                write!(f, "{}. {} 00:00:00", n, d)
            }
            AtNthWeekDay(n, d, Some(a)) => {
                write!(f, "{}. {} {}", n, d, a)
            }
        }
    }
}
impl FromStr for AtNthWeekDay {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<n>-?\d).?\w*(?P<d>[A-Za-z]*})\s*(?P<h>\d\d?(:\d\d?(:\d\d?)?)?)?"
            )
            .unwrap();
        }
        let cap = RE.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid nth weekday specifier.",
            s
        )))?;
        //
        Ok(Self(
            i8::from_str(cap.name("n").unwrap().as_str()).unwrap(),
            Weekday::from_str(cap.name("d").unwrap().as_str()).or(
                Err(ParseError(format!(
                    "`{}` is not a valid Weekday",
                    cap.name("d").unwrap().as_str()
                ))),
            )?,
            if cap.name("h").is_some() {
                Some(AtHour::from_str(
                    cap.name("h").unwrap().as_str(),
                )?)
            } else {
                None
            },
        ))
    }
}

pub enum MonthSub {
    AtDay(AtDay),
    AtNthWeekDay(AtNthWeekDay),
    AtHour(AtHour),
}
impl AtAble for MonthSub {
    fn at(&self, date: DateTime<Local>) -> DateTime<Local> {
        match self {
            Self::AtDay(at) => at.at(date),
            Self::AtNthWeekDay(at) => at.at(date),
            Self::AtHour(at) => at.at(date),
        }
    }
}
impl Display for MonthSub {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Self::AtDay(a) => write!(f, "{}", a),
            Self::AtNthWeekDay(a) => write!(f, "{}", a),
            Self::AtHour(a) => write!(f, "{}", a),
        }
    }
}
impl FromStr for MonthSub {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let nwd = AtNthWeekDay::from_str(s);
        if let Err(nwde) = nwd {
            let h = AtHour::from_str(s);
            if let Err(he) = h {
                let d = AtDay::from_str(s);
                if let Err(de) = d {
                    Err(ParseError(format!(
                        "Could not parse `MonthSub` because: {}, {}, {}",
                        nwde.0, he.0, de.0
                    )))
                } else {
                    Ok(Self::AtDay(d.unwrap()))
                }
            } else {
                Ok(Self::AtHour(h.unwrap()))
            }
        } else {
            Ok(Self::AtNthWeekDay(nwd.unwrap()))
        }
    }
}

pub struct InMonth(u32, Option<MonthSub>);
impl TimerAble for InMonth {
    fn timer(&self) -> String {
        let now = Local::now();
        let year = ((self.0 + now.month()) / 12) as i32 + now.year();
        let month = (self.0 + now.month()) % 12;

        let mut wanted = now
            .with_day(1)
            .unwrap()
            .with_year(year)
            .unwrap()
            .with_month(month)
            .unwrap();
        wanted = wanted
            .with_day(now.day().min(get_days_from_month(wanted)))
            .unwrap();

        (if let Some(at) = &self.1 {
            at.at(wanted)
        } else {
            wanted
        })
        .timer()
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Calendar {
    wd: IntRestriction,
    d: IntRestriction,
    M: IntRestriction,
    y: IntRestriction,
    s: IntRestriction,
    m: IntRestriction,
    h: IntRestriction,
}

#[derive(Debug)]
pub enum IntRestriction {
    Restrictions(IntRange),
    NoRestriction,
}

#[derive(Debug)]
pub struct IntRange(i32, i32);

#[allow(non_snake_case)]
pub struct InDuration {
    µs: Option<f64>,
    ms: Option<f64>,
    s: Option<f64>,
    m: Option<f64>,
    h: Option<f64>,
    d: Option<f64>,
    w: Option<f64>,
    M: Option<f64>,
    y: Option<f64>,
}

impl Debug for InDuration {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "In({})",
            vec![
                (self.y, "y"),
                (self.M, "M"),
                (self.w, "w"),
                (self.d, "d"),
                (self.h, "h"),
                (self.m, "m"),
                (self.s, "s"),
                (self.ms, "ms"),
                (self.µs, "µs"),
            ]
            .iter()
            .filter_map(|(o, u)| o.map(|n| format!("{}{}", n, u)))
            .collect::<Vec<String>>()
            .join(" ")
        )
    }
}

impl InDuration {
    fn to_date(&self) -> DateTime<chrono::Local> {
        Local::now()
            + chrono::Duration::microseconds(
                ((((((self.y.unwrap_or(0.0) * 365.25
                    + self.M.unwrap_or(0.0) * 30.44
                    + self.w.unwrap_or(0.0) * 7.0
                    + self.d.unwrap_or(0.0))
                    * 24.0
                    + self.h.unwrap_or(0.0))
                    * 60.0
                    + self.m.unwrap_or(0.0))
                    * 60.0
                    + self.s.unwrap_or(0.0))
                    * 1000.0
                    + self.ms.unwrap_or(0.0))
                    * 1000.0)
                    .round() as i64,
            )
    }
}

#[derive(PartialEq, Debug)]
pub enum DurationKind {
    SinceTimer,
    SinceBoot,
    SinceLogin,
    SinceSrvAct,
    SinceSrvEnd,
    IN,
}

impl TimerAble for Timer {
    fn timer(&self) -> String {
        match self {
            Timer::Timers(timers) => timers
                .iter()
                .map(|t| t.timer())
                .collect::<Vec<String>>()
                .join("\n"),
            Timer::Duration { kind, duration } => {
                let duration_string = if *kind == DurationKind::IN {
                    duration
                        .to_date()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                } else {
                    format!(
                        "{}y {}M {}w {}d {}h {}m {}s {}ms {}µs",
                        duration.y.unwrap_or(0.0),
                        duration.M.unwrap_or(0.0),
                        duration.w.unwrap_or(0.0),
                        duration.d.unwrap_or(0.0),
                        duration.h.unwrap_or(0.0),
                        duration.m.unwrap_or(0.0),
                        duration.s.unwrap_or(0.0),
                        duration.ms.unwrap_or(0.0),
                        duration.µs.unwrap_or(0.0),
                    )
                };
                match kind {
                    DurationKind::SinceTimer => {
                        format!("OnActiveSec={}", duration_string)
                    }
                    DurationKind::SinceBoot => {
                        format!("OnBootSec={}", duration_string)
                    }
                    DurationKind::SinceLogin => {
                        format!("OnStartupSec={}", duration_string)
                    }
                    DurationKind::SinceSrvAct => {
                        format!("OnUnitActiveSec={}", duration_string)
                    }
                    DurationKind::SinceSrvEnd => {
                        format!(
                            "OnUnitInactiveSec={}",
                            duration_string
                        )
                    }
                    DurationKind::IN => {
                        format!("OnCalendar={}", duration_string)
                    }
                }
            }
            Timer::In(spec) => match spec {
                In::Sec(t) => t.timer(),
                In::Min(t) => t.timer(),
                In::Hour(t) => t.timer(),
                In::Day(t) => t.timer(),
                In::Week(t) => t.timer(),
                In::Month(t) => t.timer(),
            },
            _ => todo!("timer for all Timers"),
        }
    }
}

fn get_days_from_month<T: TimeZone>(date: DateTime<T>) -> u32 {
    NaiveDate::from_ymd(
        match date.month() {
            12 => date.year() + 1,
            _ => date.year(),
        },
        match date.month() {
            12 => 1,
            _ => date.month() + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(
        date.year(),
        date.month(),
        1,
    ))
    .num_days() as u32
}

#[derive(Debug, Clone)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Timer {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        if input.contains(';') {
            input
                    .split(';')
                    .map(|i| Timer::from_str(i))
                    .collect::<Result<Vec<Timer>, <Self as FromStr>::Err>>().map(|v| Timer::Timers(v))
        } else {
            In::from_str(input).map(|i| Timer::In(i))
        }
    }
}

#[derive(PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
enum Unit {
    s = 0,
    m = 1,
    h = 2,
    d = 3,
    w = 4,
    M = 5,
    y = 6,
}
impl Unit {
    fn to_lower_or_equal(&self, other: &Self, v: u32) -> Option<u32> {
        if self == other {
            Some(v)
        } else {
            match (self, other) {
                (Unit::m, Unit::s) => Some(v * 60),
                (Unit::h, Unit::m) => Some(v * 60),
                (Unit::h, _) => {
                    Unit::m.to_lower_or_equal(other, v * 60)
                }
                (Unit::d, Unit::h) => Some(v * 24),
                (Unit::d, _) => {
                    Unit::h.to_lower_or_equal(other, v * 24)
                }
                (Unit::w, Unit::d) => Some(v * 7),
                (Unit::w, _) => {
                    Unit::d.to_lower_or_equal(other, v * 7)
                }
                (Unit::y, Unit::M) => Some(v * 12),
                _ => None,
            }
        }
    }
}
impl Display for Unit {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Unit::s => "s",
                Unit::m => "m",
                Unit::h => "h",
                Unit::d => "d",
                Unit::w => "w",
                Unit::M => "M",
                Unit::y => "y",
            }
        )
    }
}

macro_rules! match_any {
    ($value:expr, $first:expr) => {
        $value.eq_ignore_ascii_case($first)
    };
    ($value:expr, $first:expr, $($pattern:expr), +) => {
        $value.eq_ignore_ascii_case($first) || match_any!($value, $($pattern), +)
    };
}

impl FromStr for Unit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match 1 {
            _ if match_any!(s, "s", "sec", "second", "seconds") => {
                Ok(Unit::s)
            }
            _ if s == "m"
                || match_any!(s, "min", "minute", "minutes") =>
            {
                Ok(Unit::m)
            }
            _ if match_any!(s, "h", "hr", "hour", "hours") => {
                Ok(Unit::h)
            }
            _ if match_any!(s, "d", "day", "days") => Ok(Unit::d),
            _ if match_any!(s, "w", "week", "weeks") => Ok(Unit::w),
            _ if s == "M" || match_any!(s, "month", "months") => {
                Ok(Unit::M)
            }
            _ if match_any!(s, "y", "year", "years") => Ok(Unit::y),
            _ => {
                Err(ParseError(format!("{} is not a valid Unit", s)))
            }
        }
    }
}

impl FromStr for In {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lowest_unit = None;
        let mut total = 0u32;

        lazy_static! {
            static ref IN_AT: Regex =
                Regex::new(r"^in?\b(?P<in>.*?)(\bat?\b(?P<at>.*))?$")
                    .unwrap();
            static ref UNITS: Regex =
                Regex::new(r"(?P<n>\d+)\s*(?P<u>[a-zA-z])").unwrap();
        }

        let caps = IN_AT.captures(s).ok_or(ParseError(format!(
            "`{}` is not a valid `in` specification.",
            s
        )))?;

        for cap in
            UNITS.captures_iter(caps.name("in").unwrap().as_str())
        {
            let current =
                u32::from_str(cap.name("n").unwrap().as_str())
                    .unwrap();
            let unit =
                Unit::from_str(cap.name("u").unwrap().as_str())?;
            if lowest_unit.is_some()
                && unit >= *lowest_unit.as_ref().unwrap()
            {
                total += unit
                    .to_lower_or_equal(
                        &lowest_unit.as_ref().unwrap(),
                        current,
                    )
                    .ok_or(ParseError(format!(
                        "The units {} and {} are not compatible",
                        &unit,
                        &lowest_unit.as_ref().unwrap()
                    )))?
            } else {
                total = current
                    + if lowest_unit.is_some() {
                        lowest_unit
                                    .as_ref()
                                    .unwrap()
                                    .to_lower_or_equal(
                                        &unit, total,
                                    )
                                    .ok_or(ParseError(format!(
                                                "The units {} and {} are not compatible",
                                                &unit,
                                                &lowest_unit.as_ref().unwrap())))?
                    } else {
                        0
                    };
                lowest_unit = Some(unit);
            }
        }

        //.collect::<Result<(), Self::Err>>()?;
        if lowest_unit.is_none() {
            Err(ParseError(format!(
                "There was no time provided in {}",
                caps.name("in").unwrap().as_str()
            )))?;
        }

        let ats = if caps.name("at").is_some() {
            Some(caps.name("at").unwrap().as_str().trim())
        } else {
            None
        };

        Ok(match (lowest_unit, ats) {
            (Some(Unit::s), None) => In::Sec(InSec(total.into())),
            (Some(Unit::s), _) => Err(ParseError(String::from(
                "Time in seconds specified with at",
            )))?,
            (Some(Unit::m), None) => {
                In::Min(InMin(total.into(), None))
            }
            (Some(Unit::m), Some(a)) => In::Min(InMin(
                total.into(),
                Some(AtSec::from_str(a)?),
            )),
            (Some(Unit::h), None) => {
                In::Hour(InHour(total.into(), None))
            }
            (Some(Unit::h), Some(a)) => In::Hour(InHour(
                total.into(),
                Some(AtMin::from_str(a)?),
            )),
            (Some(Unit::d), None) => {
                In::Day(InDay(total.into(), None))
            }
            (Some(Unit::d), Some(a)) => In::Day(InDay(
                total.into(),
                Some(AtHour::from_str(a)?),
            )),
            (Some(Unit::w), None) => {
                In::Week(InWeek(total.into(), None))
            }
            (Some(Unit::w), Some(a)) => In::Week(InWeek(
                total.into(),
                Some(WeekSub::from_str(a)?),
            )),
            (Some(Unit::M), None) => {
                In::Month(InMonth(total.into(), None))
            }
            (Some(Unit::M), Some(a)) => In::Month(InMonth(
                total.into(),
                Some(MonthSub::from_str(a)?),
            )),

            _ => todo!(),
        })
    }
}
