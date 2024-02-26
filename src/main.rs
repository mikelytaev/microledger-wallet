use automerge::{ObjType, AutoCommit, transaction::Transactable, ReadDoc, Change, AutomergeError};
use uuid::Uuid;
use iota_sdk::client::{Client, Result};

fn doc_binary_vec(mut doc: AutoCommit) -> Vec<Vec<u8>> {
    let changes=  doc.get_changes(&[]);
    let mut binary_changes: Vec<Vec<u8>> = Vec::new();
    for mut ch in changes {
        let bts = ch.raw_bytes();
        println!("{}", bts.len());
        binary_changes.push(bts.to_owned());
    }
    binary_changes
}

fn binary_vec_to_doc(binary_changes: Vec<Vec<u8>>) -> AutoCommit {
    let mut restored_changes: Vec<Change> = Vec::new();
    for bc in binary_changes {
        match Change::try_from(bc.as_slice()) {
            Ok(ch) => {restored_changes.push(ch)}
            Err(_) => {println!("load error")}
        }
    }

    let mut doc_restored = AutoCommit::new();
    //println!("{}", restored_changes.len());
    doc_restored.apply_changes(restored_changes).unwrap();
    doc_restored
}

#[tokio::main]
async fn main() -> Result<()>  {
    let mut doc = AutoCommit::new();

    let contacts = doc.put_object(automerge::ROOT, "contacts", ObjType::Map).unwrap();
    let alice = doc.put_object(&contacts, Uuid::new_v4().to_string(),  ObjType::Map).unwrap();
    doc.put(&alice, "name", "Alice").unwrap();
    doc.put(&alice, "email", "alice@example.com").unwrap();
    doc.commit();

    let bob = doc.put_object(&contacts, Uuid::new_v4().to_string(), ObjType::Map).unwrap();
    doc.put(&bob, "name", "Bob").unwrap();
    doc.put(&bob, "email", "bob@example.com").unwrap();
    doc.commit();

    let binary_changes = doc_binary_vec(doc);
    let mut doc_restored = binary_vec_to_doc(binary_changes);

    let serialized = serde_json::to_string(&automerge::AutoSerde::from(&doc_restored)).unwrap();
    println!("{:?}", serialized);

    let client = Client::builder()
        .with_node("https://api.testnet.shimmer.network")?.finish().await?;
    let block = client
        .build_block()
        .with_data("asdfgh".as_bytes().to_vec())
        .with_tag("wethwerhw45uy54u54u54u54y54".as_bytes().to_vec())
        .finish()
        .await?;

    let block_id = block.id();

    client

    println!("{block:#?}");

    // Try to check if the block has been confirmed.
    client.retry_until_included(&block_id, None, None).await?;
    println!(
        "Block with no payload included: {}",
        block_id
    );



    Result::Ok(())
}
