use std::collections::{BTreeMap, HashMap};

type Timestamp = u64;

struct TimedValue {
    value: String,
    expires_at: Option<Timestamp>,
}

impl TimedValue {
    fn is_alive_at(&self, timestamp: Timestamp) -> bool {
        match self.expires_at {
            Some(expiry_timestamp) => timestamp < expiry_timestamp,
            None => true,
        }
    }
}

struct SnapshotValue {
    value: String,
    remaining_ttl: Option<Timestamp>,
}

type Record = BTreeMap<String, TimedValue>;
type SnapshotRecord = BTreeMap<String, SnapshotValue>;
type Snapshot = HashMap<String, SnapshotRecord>;

#[derive(Default)]
pub struct InMemoryDatabase {
    records: HashMap<String, Record>,
    backups: BTreeMap<Timestamp, Snapshot>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_at(&mut self, key: &str, field: &str, value: &str, _timestamp: Timestamp) {
        self.store(key, field, value, None);
    }

    pub fn set_at_with_ttl(
        &mut self,
        key: &str,
        field: &str,
        value: &str,
        timestamp: Timestamp,
        ttl: Timestamp,
    ) {
        self.store(key, field, value, Some(timestamp + ttl));
    }

    fn store(&mut self, key: &str, field: &str, value: &str, expires_at: Option<Timestamp>) {
        self.records.entry(key.to_owned()).or_default().insert(
            field.to_owned(),
            TimedValue { value: value.to_owned(), expires_at },
        );
    }

    pub fn get_at(&self, key: &str, field: &str, timestamp: Timestamp) -> Option<&str> {
        let stored_value = self.records.get(key)?.get(field)?;
        if stored_value.is_alive_at(timestamp) {
            Some(stored_value.value.as_str())
        } else {
            None
        }
    }

    pub fn delete_at(&mut self, key: &str, field: &str, timestamp: Timestamp) -> bool {
        let Some(record) = self.records.get_mut(key) else {
            return false;
        };
        let Some(stored_value) = record.get(field) else {
            return false;
        };
        if !stored_value.is_alive_at(timestamp) {
            return false;
        }
        record.remove(field);
        true
    }

    pub fn scan_at(&self, key: &str, timestamp: Timestamp) -> Vec<String> {
        match self.records.get(key) {
            Some(record) => format_live_fields(record.iter(), timestamp),
            None => Vec::new(),
        }
    }

    pub fn scan_by_prefix_at(&self, key: &str, prefix: &str, timestamp: Timestamp) -> Vec<String> {
        let Some(record) = self.records.get(key) else {
            return Vec::new();
        };
        let fields_with_prefix = record
            .range(prefix.to_owned()..)
            .take_while(|(field, _)| field.starts_with(prefix));
        format_live_fields(fields_with_prefix, timestamp)
    }

    pub fn backup(&mut self, timestamp: Timestamp) -> usize {
        let snapshot = self.capture_live_state(timestamp);
        let saved_record_count = snapshot.len();
        self.backups.insert(timestamp, snapshot);
        saved_record_count
    }

    pub fn restore(&mut self, timestamp: Timestamp, timestamp_to_restore: Timestamp) {
        let Some((_, snapshot)) = self.backups.range(..=timestamp_to_restore).next_back() else {
            return;
        };
        self.records = rebuild_records(snapshot, timestamp);
    }

    fn capture_live_state(&self, timestamp: Timestamp) -> Snapshot {
        self.records
            .iter()
            .filter_map(|(key, record)| {
                let live_fields = capture_live_fields(record, timestamp);
                if live_fields.is_empty() {
                    None
                } else {
                    Some((key.clone(), live_fields))
                }
            })
            .collect()
    }
}

fn format_live_fields<'a>(
    record_fields: impl Iterator<Item = (&'a String, &'a TimedValue)>,
    timestamp: Timestamp,
) -> Vec<String> {
    record_fields
        .filter(|(_, stored_value)| stored_value.is_alive_at(timestamp))
        .map(|(field, stored_value)| format!("{field}({})", stored_value.value))
        .collect()
}

fn capture_live_fields(record: &Record, timestamp: Timestamp) -> SnapshotRecord {
    record
        .iter()
        .filter(|(_, stored_value)| stored_value.is_alive_at(timestamp))
        .map(|(field, stored_value)| {
            let remaining_ttl = stored_value
                .expires_at
                .map(|expiry_timestamp| expiry_timestamp - timestamp);
            (field.clone(), SnapshotValue { value: stored_value.value.clone(), remaining_ttl })
        })
        .collect()
}

fn rebuild_records(snapshot: &Snapshot, timestamp: Timestamp) -> HashMap<String, Record> {
    snapshot
        .iter()
        .map(|(key, snapshot_record)| (key.clone(), rebuild_record(snapshot_record, timestamp)))
        .collect()
}

fn rebuild_record(snapshot_record: &SnapshotRecord, timestamp: Timestamp) -> Record {
    snapshot_record
        .iter()
        .map(|(field, snapshot_value)| {
            let expires_at = snapshot_value
                .remaining_ttl
                .map(|remaining_ttl| timestamp + remaining_ttl);
            (field.clone(), TimedValue { value: snapshot_value.value.clone(), expires_at })
        })
        .collect()
}

fn main() {
    let mut database = InMemoryDatabase::new();

    database.set_at("user:1", "name", "ana", 1);
    database.set_at("user:1", "city", "cebu", 1);
    database.set_at("user:1", "nickname", "an", 1);

    check("get name at 2", database.get_at("user:1", "name", 2), Some("ana"));
    check("get email at 2", database.get_at("user:1", "email", 2), None);
    check(
        "scan at 2",
        database.scan_at("user:1", 2),
        vec!["city(cebu)", "name(ana)", "nickname(an)"],
    );
    check(
        "scan prefix n at 2",
        database.scan_by_prefix_at("user:1", "n", 2),
        vec!["name(ana)", "nickname(an)"],
    );

    check("delete city at 3", database.delete_at("user:1", "city", 3), true);
    check("delete city again at 4", database.delete_at("user:1", "city", 4), false);

    database.set_at_with_ttl("user:1", "session", "token", 5, 10);
    check("get session at 9", database.get_at("user:1", "session", 9), Some("token"));
    check("get session at 15", database.get_at("user:1", "session", 15), None);
    check("scan at 15", database.scan_at("user:1", 15), vec!["name(ana)", "nickname(an)"]);

    check("backup at 8", database.backup(8), 1);
    database.set_at("user:1", "name", "bea", 20);
    check("get name at 20", database.get_at("user:1", "name", 20), Some("bea"));

    database.restore(30, 8);
    check("get name at 30", database.get_at("user:1", "name", 30), Some("ana"));
    check("get session at 36", database.get_at("user:1", "session", 36), Some("token"));
    check("get session at 37", database.get_at("user:1", "session", 37), None);
}

fn check<T, U>(case_label: &str, actual_result: T, expected_result: U)
where
    T: std::fmt::Debug + PartialEq<U>,
    U: std::fmt::Debug,
{
    assert_eq!(actual_result, expected_result);
    println!("{case_label} = {actual_result:?}");
}
