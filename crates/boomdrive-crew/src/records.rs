//! crew's recutils files, read as crew reads them: through weir's reader,
//! each record with the record set the descriptor before it names, a
//! value continued on `+` lines joined by newlines, comments and
//! descriptors left out.

use weir_capture::rec::{self, Record};

/// A file's records, each with the record set its descriptor names, none
/// for a record before any `%rec`.
pub type Sets = Vec<(Option<String>, Record)>;

/// The records of `text`, or why it is not recutils.
pub fn read(text: &str) -> Result<Sets, String> {
    let mut set = None;
    let mut out = Vec::new();
    for record in rec::read(text).map_err(|error| error.to_string())? {
        match record.get("%rec") {
            Some(named) => set = named.split_whitespace().next().map(str::to_string),
            None if record
                .fields()
                .iter()
                .all(|(name, _)| name.starts_with('%')) => {}
            None => out.push((set.clone(), record)),
        }
    }
    Ok(out)
}

/// The records of `sets` in the record set `set`, in file order.
pub fn of<'a>(sets: &'a Sets, set: &'a str) -> impl Iterator<Item = &'a Record> + 'a {
    sets.iter()
        .filter(move |(named, _)| named.as_deref() == Some(set))
        .map(|(_, record)| record)
}

/// A field's text as a whole number, none where it holds anything else.
pub fn whole(text: &str) -> Option<u64> {
    (!text.is_empty() && text.bytes().all(|b| b.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
}

/// The records as recutils text, under `descriptor`: the descriptor, then
/// each record after a blank line, as crew's own tests write a runner's
/// files.
pub fn under(descriptor: &str, records: &[Record]) -> String {
    match records.is_empty() {
        true => descriptor.to_string(),
        false => format!("{descriptor}\n{}", rec::write(records)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_are_read_with_their_sets_and_descriptors_left_out() {
        let text = "# a comment\n\n%rec: Place\n%key: Request\n\nRequest: 1\nStatus: {\"a\":1}\n\n\
                    %rec: Run\n%key: Address\n\nAddress: madswan/planner\nWords: one\n+ two\n";
        let sets = read(text).unwrap();
        assert_eq!(sets.len(), 2);
        assert_eq!(of(&sets, "Place").count(), 1);
        let run = of(&sets, "Run").next().unwrap();
        assert_eq!(run.get("Words"), Some("one\ntwo"));
        assert_eq!(read("Request: 1\n").unwrap()[0].0, None);
    }

    #[test]
    fn a_whole_number_is_digits_alone() {
        assert_eq!(whole("12"), Some(12));
        for text in ["", "-1", "1.5", "x", " 3"] {
            assert_eq!(whole(text), None, "{text:?}");
        }
    }
}
