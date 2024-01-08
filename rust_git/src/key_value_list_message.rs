use std::collections::HashMap;
use bytes::Bytes;

#[derive(Debug, PartialEq)]
pub struct KeyValuePairList {
    pub data: HashMap<String, KeyValuePairEntry>
}

#[derive(Debug, PartialEq)]
pub enum KeyValuePairEntry {
    Singleton(Bytes),
    List(Vec<Bytes>)
}

impl KeyValuePairList {
    pub fn new() -> Self {
        KeyValuePairList { data: HashMap::new() }
    }

    ///
    /// ```
    /// use std::collections::HashMap;
    /// use bytes::Bytes;
    /// use rust_git::key_value_list_message::{KeyValuePairEntry, KeyValuePairList};
    /// let input = Bytes::from("firstkey firstvalue\nsecondkey secondvalue\n");
    ///
    /// let output = KeyValuePairList::from(input).unwrap();
    ///
    /// let value1 = Bytes::from("firstvalue");
    /// let value2 = Bytes::from("secondvalue");
    /// let mut expected_data_inner = HashMap::new();
    /// expected_data_inner.insert(String::from("firstkey"), KeyValuePairEntry::Singleton(value1));
    /// expected_data_inner.insert(String::from("secondkey"), KeyValuePairEntry::Singleton(value2));
    /// let expected_data = KeyValuePairList { data: expected_data_inner };
    /// assert_eq!(output, expected_data);
    /// ```
    pub fn from(input: Bytes) -> Result<Self, &'static str> {
        let mut data: HashMap<String, KeyValuePairEntry> = HashMap::new();

        let mut complete = false;
        let mut start = 0;
        let mut input_remaining = input.clone();
        while !complete {
            start = 0;
            let space_idx = input_remaining.iter().position(|&b| b == b' ');
            let newline_idx = input_remaining.iter().position(|&b| b == b'\n');

            println!("space_idx {:?}, newline_idx {:?}", space_idx, newline_idx);

            // TODO: cleanup dupe branches
            match (space_idx, newline_idx) {
                (None, Some(newline)) => {
                    if newline != start {
                        return Err("Newline and start incompatible");
                    }

                    let val_to_put = KeyValuePairEntry::Singleton(input_remaining.slice(start+1..));
                    data.insert(String::from("NONE"), val_to_put);
                    complete = true;
                },
                (Some(space), Some(newline)) if newline < space => {
                    if newline != start {
                    return Err("Newline and start incompatible");
                    }

                    let val_to_put = KeyValuePairEntry::Singleton(input_remaining.slice(start+1..));
                    data.insert(String::from("NONE"), val_to_put);
                    complete = true;
                }
                _ => { }
            }

            if complete {
                break;
            }

            let space_idx = space_idx.unwrap();

            let key = input_remaining.slice(start..space_idx);
            let key_string = String::from_utf8(key.to_vec()).unwrap();

            println!("Found key string: {:?}", key_string);

            let mut end = start;
            loop {
                let to_search = input_remaining.slice(end+1..);
                end = to_search.iter().position(|&b| b == b'\n').unwrap();

                if to_search.get(end+1) == None {
                    break;
                }

                println!("Comparing {} to {}", *to_search.get(end+1).unwrap(), b' ');
                if *to_search.get(end+1).unwrap() != b' ' {
                    break;
                }
            }

            // TODO: add code to drop leading space on continuations e.g. replace(b'\n ', b'\n')
            // note the end+1 in python the end is inclusive, in rust we have to make it inclusive by adding 1
            let val_to_add = input_remaining.slice(space_idx+1..end+1);

            println!("Found value string: {:?}", val_to_add);

            if data.contains_key(&key_string) {
                let val = data.get(&key_string).unwrap();

                let updated_val = match val {
                    KeyValuePairEntry::Singleton(singleton_val) => {
                        KeyValuePairEntry::List(vec![singleton_val.clone(), val_to_add])
                    },
                    KeyValuePairEntry::List(list_val) => {
                        let mut list_val_to_update = list_val.clone();
                        list_val_to_update.push(val_to_add);
                        KeyValuePairEntry::List(list_val_to_update)
                    }
                };
                data.insert(key_string, updated_val);
            } else {
                data.insert(key_string, KeyValuePairEntry::Singleton(val_to_add));
            }

            start = end + 1 + 1;

            if start == input_remaining.len() {
                complete = true;
            }

            // TODO: in python impl the initial space and nl find starts at start and returns -1 if fail
            //       in rust I need to update this to work by updating teh data to search
            input_remaining = input_remaining.slice(start..);

            // TODO: turn the key into an emum of Option<String> for the empty case

            // TODO: the start needs updating by the offset or resetting to zero each time
        }

        Ok(KeyValuePairList { data })
    }
}