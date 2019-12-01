#[cfg(feature = "stream")]
use async_std;
#[cfg(feature = "stream")]
use brontide::BrontideBuilder;
#[cfg(feature = "stream")]
use futures::StreamExt;

//TODO biggest thing to test with this is that streams can work out of order.
// Stream A connects
// Stream B connects
// Stream A sends act_one
// We return act_two
// Stream B sends act_one
// We return act_two
// Stream B sends act_three
// Stream B should be the first ready in the above scenario.

////TODO Tests we need
////1. Test sending in a message without doing a handshake. Ensure we fail on the stream side.
////2. Test timeouts.
////3. Test stream returns
////4. Test invalid keys.
////5. Test opening up a stream, and then not having the handshake done.
////If Handshake is complete is some time period, the stream should auto-rewake and read messages.

#[cfg(feature = "stream")]
#[async_std::test]
async fn test_brontide_stream() {
    //TODO break this into it's own setup.
    async_std::task::spawn(async move {
        let listener = async_std::net::TcpListener::bind("0.0.0.0:13038")
            .await
            .unwrap();

        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            // let stream = stream?;
            let stream = stream.unwrap();

            async_std::task::spawn(async move {
                let mut accepted_stream =
                    BrontideBuilder::new([1; 32]).accept(stream).await.unwrap();
                accepted_stream.write(b"hello").await.unwrap();
                // accepted_stream.write(b"olleh").await;
                // accepted_stream.write(b"hello").await;
                // loop {}
            });
        }
    });
    let mut pub_key = [0_u8; 33];
    pub_key.copy_from_slice(
        &hex::decode("031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f").unwrap(),
    );

    let mut stream = BrontideBuilder::new([1; 32])
        .connect("0.0.0.0:13038", pub_key)
        .await
        .unwrap();

    while let Some(packet) = stream.next().await {
        assert_eq!(packet, b"hello");
    }
}
//

//For setups, rip out the listener function and return a channel to it. Then we can write to the
//channel and it will pop up in the stream. TODO
