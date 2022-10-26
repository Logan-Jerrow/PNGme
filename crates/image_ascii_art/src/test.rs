use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{collections::HashMap, fmt::Display};

fn main() -> Result<(), Box<dyn Error>> {
    let mut m = HashMap::new();
    m.insert("First", "a");
    m.insert("Second.1", "b");
    m.insert("Second.2", "c");
    m.insert("Second.3", "d");
    m.insert("Third.1", "e");
    m.insert("Forth", "f");

    let v = merge(m.into_iter().map(|(s, v)| (s.to_owned(), v)))?;

    let v = dbg!(v);

    let s = serde_json::to_string(&v).unwrap();

    dbg!(s);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MyValue<T> {
    Compound(HashMap<String, T>),
    Value(T),
}

impl<T> Default for MyValue<T> {
    fn default() -> Self {
        MyValue::Compound(HashMap::new())
    }
}

#[derive(Debug, Clone, Copy)]
enum MergeError {
    TypeCollision,
    KeyCollision,
}

impl Display for MergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergeError::KeyCollision => {
                write!(f, "Two key holding a normal value collide.")
            }
            MergeError::TypeCollision => {
                write!(f, "Two key holding different type of value collide.")
            }
        }
    }
}

impl Error for MergeError {}

fn merge<I, K, T>(iter: I) -> Result<HashMap<String, MyValue<T>>, MergeError>
where
    I: IntoIterator<Item = (K, T)>,
    K: AsRef<str>,
{
    iter.into_iter()
        .try_fold(HashMap::new(), |mut m, (label, value)| {
            if let Some((root_label, child_label)) = label.as_ref().split_once(".") {
                if let Some(m) = m.get_mut(root_label) {
                    if let MyValue::Compound(m) = m {
                        if m.insert(child_label.into(), value).is_some() {
                            return Err(MergeError::KeyCollision);
                        };
                    } else {
                        return Err(MergeError::TypeCollision);
                    }
                } else {
                    let cm = Some((child_label.into(), value)).into_iter().collect();
                    m.insert(root_label.into(), MyValue::Compound(cm));
                }
            } else if let Some(old) = m.insert(label.as_ref().into(), MyValue::Value(value)) {
                let e = match old {
                    MyValue::Compound(_) => MergeError::TypeCollision,
                    MyValue::Value(_) => MergeError::KeyCollision,
                };
                return Err(e);
            }
            Ok(m)
        })
}
