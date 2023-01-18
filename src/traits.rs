use chrono::{DateTime, Utc, NaiveDateTime};
use hdk::prelude::{AnyLinkableHash, ExternResult, Record};

pub trait IndexableEntry {
    ///Time that entry type this trait is implemented on should be indexed under
    fn entry_time(&self) -> DateTime<Utc>;
    fn hash(&self) -> ExternResult<AnyLinkableHash>;
}

impl IndexableEntry for Record {
    fn entry_time(&self) -> DateTime<Utc> {
        let (seconds, nanos) = self.action_hashed().timestamp().as_seconds_and_nanos();

        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(seconds, nanos).unwrap(), 
            Utc
        )
    }
    fn hash(&self) -> ExternResult<AnyLinkableHash> {
        Ok(AnyLinkableHash::from(self.action_hashed().clone().hash))
    }
}
