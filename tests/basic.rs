use color_eyre::Result;
use common::{connect, send_receive};
use serial_keel::{
    actions::{Action, Response},
    endpoint::EndpointLabel,
    error::Error,
};

mod common;

#[tokio::test]
async fn can_connect() -> Result<()> {
    connect().await?;

    Ok(())
}

#[tokio::test]
async fn can_send_and_receive() -> Result<()> {
    let mut client = connect().await?;
    let _response = send_receive(&mut client, "hi".into()).await?;

    Ok(())
}

#[tokio::test]
async fn non_json_request_is_bad() -> Result<()> {
    let mut client = connect().await?;
    let response = send_receive(&mut client, "hi".into()).await?;

    assert!(matches!(response, Result::Err(Error::BadRequest(_))));

    Ok(())
}

#[tokio::test]
async fn non_existing_mock_endpoint_observe_is_ok() -> Result<()> {
    serial_keel::logging::init().await;
    let mut client = connect().await?;

    let request = Action::observe_mock("non_existing_mock_endpoint_observe_is_ok").serialize();
    let response = send_receive(&mut client, request).await?;

    assert!(matches!(response, Result::Ok(Response::Ok)));

    Ok(())
}

#[tokio::test]
async fn observe_same_twice_is_bad() -> Result<()> {
    serial_keel::logging::init().await;

    let mut client = connect().await?;

    let request = Action::observe_mock("twice").serialize();
    let response = send_receive(&mut client, request).await?;
    assert!(matches!(response, Result::Ok(Response::Ok)));

    let request = Action::observe_mock("twice").serialize();
    let response = send_receive(&mut client, request).await?;
    assert!(matches!(response, Result::Err(Error::BadRequest(_))));

    Ok(())
}

#[tokio::test]
async fn observe_mock_and_write_is_bad_no_control() -> Result<()> {
    serial_keel::logging::init().await;

    let mut client = connect().await?;

    let label = EndpointLabel::mock("some-mock");

    let request = Action::Observe(label.clone()).serialize();
    let response = send_receive(&mut client, request).await?;
    assert!(matches!(response, Result::Ok(Response::Ok)));

    let request = Action::write(&label, "Hi there".into()).serialize();
    let response = send_receive(&mut client, request).await?;

    assert_ne!(response, Ok(Response::Ok));

    Ok(())
}
