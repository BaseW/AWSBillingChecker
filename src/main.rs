use aws_config::meta::region::RegionProviderChain;
use aws_lambda_events::event::cloudwatch_logs::{CloudwatchLogsEvent, CloudwatchLogsRawData};
use aws_sdk_billingconductor::Client;
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

async fn pricing_rules_handler(event: LambdaEvent<CloudwatchLogsEvent>) -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let list_pricing_rules = client.list_pricing_rules();
    let pricing_rules_result = list_pricing_rules.send().await;
    match pricing_rules_result {
        Ok(pricing_rules) => println!("pricing_rules: {:?}", pricing_rules),
        Err(err) => println!("err: {:?}", err),
    };

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

    // function_handler(event)
    //     .await
    //     .expect("failed to handle event");
    pricing_rules_handler(event).await.expect("failed to fetch");
}
