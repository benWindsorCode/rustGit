use std::collections::HashMap;
use bytes::Bytes;

#[derive(Debug, PartialEq)]
pub struct KeyValuePairList {
    pub data: HashMap<KeyValuePairKey, KeyValuePairEntry>,
    // pub key_list: Vec<String>
}

#[derive(Debug, PartialEq)]
pub enum KeyValuePairEntry {
    Singleton(Bytes),
    List(Vec<Bytes>)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyValuePairKey {
    Contents,
    Key(String)
}

impl KeyValuePairList {
    pub fn new() -> Self {
        // KeyValuePairList { data: HashMap::new(), key_list: Vec::new() }
        KeyValuePairList { data: HashMap::new() }
    }

    pub fn insert_contents(&mut self, contents: Bytes) {
        self.data.insert(KeyValuePairKey::Contents, KeyValuePairEntry::Singleton(contents));
    }

    pub fn insert_pair(&mut self, key_string: String, val_to_add: Bytes) {
        let key = KeyValuePairKey::Key(key_string);
        if self.data.contains_key(&key) {
            let val = self.data.get(&key).unwrap();

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
            self.data.insert(key, updated_val);
        } else {
            self.data.insert(key, KeyValuePairEntry::Singleton(val_to_add));
        }
    }

    ///
    /// Parse key value pairs from input data.
    ///
    /// New lines separate entries.
    /// New line followed by a space is a continuation of the previous entry.
    /// A space separates the key and value.
    ///
    /// e.g.:
    /// ```
    /// use std::collections::HashMap;
    /// use bytes::Bytes;
    /// use rust_git::key_value_list_message::{KeyValuePairEntry, KeyValuePairKey, KeyValuePairList};
    /// let input = Bytes::from("firstkey firstvalue\nsecondkey secondvalue\n");
    ///
    /// let output = KeyValuePairList::from(input).unwrap();
    ///
    /// let value1 = Bytes::from("firstvalue");
    /// let value2 = Bytes::from("secondvalue");
    /// let mut expected_data_inner = HashMap::new();
    /// expected_data_inner.insert(KeyValuePairKey::Key(String::from("firstkey")), KeyValuePairEntry::Singleton(value1));
    /// expected_data_inner.insert(KeyValuePairKey::Key(String::from("secondkey")), KeyValuePairEntry::Singleton(value2));
    /// let expected_data = KeyValuePairList { data: expected_data_inner };
    /// assert_eq!(output, expected_data);
    /// ```
    ///
    /// Multi line values are supported by adding spaces at the start of a line, where these get stripped out when
    /// we parse the continued value
    /// e.g.:
    /// ```
    /// use std::collections::HashMap;
    /// use bytes::Bytes;
    /// use rust_git::key_value_list_message::{KeyValuePairEntry, KeyValuePairKey, KeyValuePairList};
    /// let input = Bytes::from("firstkey firstvalue\n continuation of value\n further continuation\nsecondkey secondvalue\n");
    ///
    /// let output = KeyValuePairList::from(input).unwrap();
    ///
    /// let value1 = Bytes::from("firstvalue\ncontinuation of value\nfurther continuation\n");
    /// let value2 = Bytes::from("secondvalue");
    /// let mut expected_data_inner = HashMap::new();
    /// expected_data_inner.insert(KeyValuePairKey::Key(String::from("firstkey")), KeyValuePairEntry::Singleton(value1));
    /// expected_data_inner.insert(KeyValuePairKey::Key(String::from("secondkey")), KeyValuePairEntry::Singleton(value2));
    /// let expected_data = KeyValuePairList { data: expected_data_inner };
    /// assert_eq!(output, expected_data);
    /// ```
    pub fn from(input: Bytes) -> Result<Self, &'static str> {
        // let mut data: HashMap<String, KeyValuePairEntry> = HashMap::new();
        let mut data = KeyValuePairList::new();

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

                    data.insert_contents(input_remaining.slice(start+1..));
                    complete = true;
                },
                (Some(space), Some(newline)) if newline < space => {
                    if newline != start {
                    return Err("Newline and start incompatible");
                    }

                    data.insert_contents(input_remaining.slice(start+1..));
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
                println!("searching {}", String::from_utf8(to_search.to_vec()).unwrap());

                // find the nearest newline starting from the end of last search (but adding back the offset so that we count correctly)
                end = input_remaining.iter().skip(end+1).position(|&b| b == b'\n').unwrap() + end + 1;

                if input_remaining.get(end+1) == None {
                    break;
                }

                println!("Comparing {} to {}", *input_remaining.get(end+1).unwrap(), b' ');
                if *input_remaining.get(end+1).unwrap() != b' ' {
                    break;
                }
            }

            // note the end+1 in python the end is inclusive, in rust we have to make it inclusive by adding 1
            let val_to_add = input_remaining.slice(space_idx+1..end+1);

            let formatted_val_to_add = String::from_utf8(val_to_add.to_vec()).unwrap();
            let formatted_val_to_add = formatted_val_to_add.replace("\n ", "\n");

            let val_to_add = Bytes::from(formatted_val_to_add);

            println!("Found value string: {:?}", val_to_add);

            data.insert_pair(key_string, val_to_add);

            start = end + 1;

            if start == input_remaining.len() {
                complete = true;
            }

            // TODO: in python impl the initial space and nl find starts at start and returns -1 if fail
            //       in rust I need to update this to work by updating teh data to search
            input_remaining = input_remaining.slice(start..);

            // TODO: turn the key into an emum of Option<String> for the empty case

            // TODO: the start needs updating by the offset or resetting to zero each time
        }

        Ok(data)
    }

    ///
    /// ```
    /// use std::collections::HashMap;
    /// use bytes::Bytes;
    /// use rust_git::key_value_list_message::{KeyValuePairEntry, KeyValuePairKey, KeyValuePairList};
    /// let value1 = Bytes::from("firstvalue");
    /// let value2 = Bytes::from("secondvalue");
    /// let mut input_data_inner = HashMap::new();
    /// input_data_inner.insert(KeyValuePairKey::Key(String::from("firstkey")), KeyValuePairEntry::Singleton(value1));
    /// input_data_inner.insert(KeyValuePairKey::Key(String::from("secondkey")), KeyValuePairEntry::Singleton(value2));
    /// let input_data = KeyValuePairList { data: input_data_inner };
    ///
    /// let output = input_data.into_string();
    /// let expected = "firstkey firstvalue\nsecondkey secondvalue\n";
    /// assert_eq!(output, expected);
    /// ```
    ///
    ///
    pub fn into_string(&self) -> String {
        // TODO: the implementation needs to be ordered on insertion order ideally to preserve the output every time
        let mut output = String::from("");

        for (key, value) in &self.data {
            if key == &KeyValuePairKey::Contents {
                continue;
            }

            let vals_to_write = match value {
                KeyValuePairEntry::Singleton(value_single) => vec![String::from_utf8(value_single.to_vec()).unwrap()],
                KeyValuePairEntry::List(value_list) => value_list.iter().map(|inner| String::from_utf8(inner.to_vec()).unwrap()).collect()
            };

            // TODO: handle multi line values by replacing '\n' with '\n ' in val to write
            for val_to_write in vals_to_write {
                let formatted_val_to_write = format!("{:?} {}\n", key, &val_to_write);
                output = format!("{}{}", output, formatted_val_to_write);
            }

            println!("Output is: {}", output);
        }

        let contents = match &self.data.get(&KeyValuePairKey::Contents) {
            None => Bytes::from(""),
            Some(entry) => match entry {
                KeyValuePairEntry::Singleton(entry_singleton) => entry_singleton,
                KeyValuePairEntry::List(_) => panic!("Doesnt handle list of content yet")
            }.clone()
        };

        if contents.len() > 0 {
            output = format!("{}\n{}\n", output, String::from_utf8(contents.to_vec()).unwrap());
        }

        String::from(output)
    }
}