use std::time::Duration;

use hdk3::prelude::*;

#[hdk_entry(id = "time_chunk", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct TimeChunk {
    pub from: Duration,
    pub until: Duration,
}

#[hdk_entry(id = "year_index", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Eq, PartialEq)]
pub struct YearIndex(pub u32);

#[hdk_entry(id = "month_index", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct MonthIndex(pub u32);

#[hdk_entry(id = "day_index", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct DayIndex(pub u32);

#[hdk_entry(id = "hour_index", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct HourIndex(pub u32);

#[hdk_entry(id = "minute_index", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct MinuteIndex(pub u32);

#[hdk_entry(id = "second_index", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct SecondIndex(pub u32);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TimeIndex {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}