// TODO: Testcases!
//
// Especially:
//
// 1. Available, hold it
// 2. Busy, get queued
// 3. Busy, also queed
// 4. First in queue drops
// 5. Semaphore dropped
// 6. Check that first in queue is ignored, second in place gets it
//
// Also check things like if there was a queue, but all queuers dropped, _then_ someone arrives.
// And so on.

// use serial_keel::{
//     actions::{Action, Response},
//     endpoint::EndpointLabel,
// };
// use tracing::info;
// use common::{connect, receive, send_receive};

mod common;

// Feature: Can't test queuing if endpoints are not shared
#[cfg(feature = "mocks-share-endpoints")]
mod queuing {
    use super::common::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;
    use serial_keel::{
        actions::{Action, Response},
        endpoint::EndpointLabel,
    };

    #[tokio::test]
    async fn second_user_is_queued() -> Result<()> {
        serial_keel::logging::init().await;

        // Shared data
        let label = EndpointLabel::Mock("lorem_one_word".into());
        let request = Action::control(&label).serialize();

        let port = start_server().await;

        // Client 1
        let mut client_1 = connect(port).await?;
        let response = send_receive(&mut client_1, request.clone()).await??;

        let label_clone = label.clone();
        assert_eq!(Response::ControlGranted(label_clone), response);

        // Client 2
        let mut client_2 = connect(port).await?;
        let response = send_receive(&mut client_2, request).await??;

        assert_eq!(Response::ControlQueue(label), response);

        Ok(())
    }
}
