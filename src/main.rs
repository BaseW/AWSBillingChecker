use aws_lambda_events::event::cloudwatch_logs::{CloudwatchLogsEvent, CloudwatchLogsRawData};
use lambda_runtime::{self, run, service_fn, Error, LambdaEvent};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<CloudwatchLogsEvent>) -> Result<(), Error> {
    // Extract some useful information from the request

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

#[tokio::test]
async fn test_my_lambda_handler() {
    let aws_logs = CloudwatchLogsRawData {
        data: Some("".to_string()),
    };
    let input = CloudwatchLogsEvent { aws_logs };
    let context = lambda_runtime::Context::default();

    let event = lambda_runtime::LambdaEvent::new(input, context);

    function_handler(event)
        .await
        .expect("failed to handle event");
}
