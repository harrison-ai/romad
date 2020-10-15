/// Test client
///
extern crate romad;
use romad::client::RomadClient;

use tokio::runtime::Runtime;

#[test]
fn test_list_jobs() {
    let mut client = RomadClient::new("http://13.211.203.225", "4646", None, 0).unwrap();

    let mut rt = Runtime::new().unwrap();
    let result = rt.block_on(client.list_jobs(None, None)).unwrap();

    println!("{:?}", result);
}
