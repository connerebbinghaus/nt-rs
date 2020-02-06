#[cfg(feature = "websocket")]
use nt::*;

#[cfg(feature = "websocket")]
#[tokio::main]
async fn main() -> Result<()> {
    use std::thread;
    use std::time::Duration;

    let nt = NetworkTables::connect_ws("ws://127.0.0.1:1735", "nt-ws").await?;

    let mut i = 0;
    loop {
        println!("RUNNING LOOP");
        nt.entries()
            .iter()
            .for_each(|(id, entry)| match entry.value {
                EntryValue::RpcDefinition(RpcDefinition::V0) => {
                    nt.call_rpc(*id, (0..(i % 20)).collect(), |res| {
                        println!("RECEIVED RESPONSE: {:?}", res);
                    })
                }
                _ => {}
            });
        thread::sleep(Duration::from_millis(100));
        i += 1;
    }
}

#[cfg(not(feature = "websocket"))]
fn main() {
    panic!("This example needs the \"websocket\" feature!")
}
