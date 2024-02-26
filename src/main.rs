use automerge::{ObjType, AutoCommit, transaction::Transactable, ReadDoc, Change, AutomergeError};

fn to_binary_vec(mut doc: AutoCommit) -> Vec<Vec<u8>> {
    let changes=  doc.get_changes(&[]);
    let mut binary_changes: Vec<Vec<u8>> = Vec::new();
    for mut ch in changes {
        let bts = ch.raw_bytes();
        println!("{}", bts.len());
        binary_changes.push(bts.to_owned());
    }
    binary_changes
}

fn main() -> Result<(), AutomergeError> {
    let mut doc = AutoCommit::new();

    let contacts = match doc.put_object(automerge::ROOT, "contacts", ObjType::List) {
        Ok(id) => {id}
        Err(err) => {return Result::Err(err)}
    };

    let alice = match doc.insert_object(&contacts, 0, ObjType::Map) {
        Ok(id) => {id}
        Err(err) => {return Result::Err(err)}
    };

// Finally we can set keys in the "alice" map
    doc.put(&alice, "name", "Alice")?;
    doc.commit();
    doc.put(&alice, "email", "alice@example.com")?;
    doc.commit();

    //let ch = doc.commit().unwrap();
    //let c = doc.get_change_by_hash(&ch).unwrap();

// Create another contact
    let bob = doc.insert_object(&contacts, 1, ObjType::Map)?;
    doc.put(&bob, "name", "Bob")?;
    doc.put(&bob, "email", "bob@example.com")?;
    doc.commit();

    let heads = doc.get_heads();
    let changes=  doc.get_changes(&[]);

    let mut binary_changes: Vec<&[u8]> = Vec::new();
    for mut ch in changes {
        let bts = ch.raw_bytes();
        println!("{}", bts.len());
        binary_changes.push(bts);
    }

    let mut restored_changes: Vec<Change> = Vec::new();
    for bc in binary_changes {
        match Change::try_from(bc) {
            Ok(ch) => {restored_changes.push(ch)}
            Err(_) => {println!("load error")}
        }
    }

    let mut doc_restored = AutoCommit::new();
    println!("{}", restored_changes.len());
    doc_restored.apply_changes(restored_changes)?;

    let serialized = serde_json::to_string(&automerge::AutoSerde::from(&doc_restored)).unwrap();

    println!("{:?}", serialized);
    Result::Ok(())
}
