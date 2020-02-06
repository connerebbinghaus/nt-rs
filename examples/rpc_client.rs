use nt::*;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = NetworkTables::connect("127.0.0.1:1735", "nt-rs").await?;

    let mut i = 0;
    loop {
        println!("RUNNING LOOP");
        client.entries().iter().for_each(|(id, entry)| {
            if let EntryValue::RpcDefinition(RpcDefinition::V0) = entry.value {
                client.call_rpc(*id, (0..(i % 20)).collect(), |res| {
                    println!("RECEIVED RESPONSE: {:?}", res);
                })
            }
        });
        thread::sleep(Duration::from_millis(100));
        i += 1;
    }
}
