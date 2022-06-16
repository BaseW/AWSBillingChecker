use aws_config::meta::region::RegionProviderChain;
use aws_lambda_events::event::cloudwatch_logs::CloudwatchLogsEvent;
use aws_sdk_costexplorer::{
    model::{DateInterval, Granularity},
    Client,
};
use lambda_runtime::{self, run, service_fn, Error, LambdaEvent};

async fn example_handler(event: LambdaEvent<CloudwatchLogsEvent>) -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let start_date: String = "2022-01-01".to_string();
    let end_date: String = "2022-06-01".to_string();

    let date_interval_builder = DateInterval::builder();
    let date_interval = date_interval_builder
        .set_start(Some(start_date))
        .set_end(Some(end_date))
        .build();

    let granularity = Granularity::Monthly;
    let mut metrics = Vec::<String>::new();
    metrics.push("UnblendedCost".to_string());

    let get_cost_and_usage = client
        .get_cost_and_usage()
        .set_time_period(Some(date_interval))
        .set_granularity(Some(granularity))
        .set_metrics(Some(metrics));
    let cost_and_usage_result = get_cost_and_usage.send().await;
    match cost_and_usage_result {
        Ok(cost_and_usage) => {
            // println!("cost_and_usage: {:?}", cost_and_usage);
            match cost_and_usage.results_by_time {
                Some(results_by_time) => {
                    // println!("results_by_time: {:?}", results_by_time);
                    for result_by_time in results_by_time {
                        match result_by_time.time_period {
                            Some(date_interval) => {
                                // println!("date_interval: {:?}", date_interval);
                                if let Some(start_date) = date_interval.start {
                                    if let Some(end_date) = date_interval.end {
                                        match result_by_time.total {
                                            Some(total) => {
                                                // println!("total: {:?}", total);
                                                match total.get("UnblendedCost") {
                                                    Some(unblended_cost) => {
                                                        // println!("unblended_cost: {:?}", unblended_cost);
                                                        if let Some(amount) =
                                                            unblended_cost.amount.as_ref()
                                                        {
                                                            if let Some(unit) =
                                                                unblended_cost.unit.as_ref()
                                                            {
                                                                let owned_amount =
                                                                    amount.as_str().to_string();
                                                                let owned_unit =
                                                                    unit.as_str().to_string();
                                                                println!(
                                                                    "{:?}: {:?}",
                                                                    start_date
                                                                        + " ~ "
                                                                        + end_date.as_str(),
                                                                    owned_amount
                                                                        + " "
                                                                        + owned_unit.as_str()
                                                                );
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        println!("no unblended cost");
                                                    }
                                                }
                                            }
                                            _ => {
                                                println!("no total");
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                println!("no date_interval");
                            }
                        }
                    }
                }
                _ => {
                    println!("no result");
                }
            }
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }

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

    run(service_fn(example_handler)).await
}

#[tokio::test]
async fn test_my_lambda_handler() {
    use aws_lambda_events::event::cloudwatch_logs::CloudwatchLogsRawData;

    let aws_logs = CloudwatchLogsRawData {
        data: Some("".to_string()),
    };
    let input = CloudwatchLogsEvent { aws_logs };
    let context = lambda_runtime::Context::default();

    let event = lambda_runtime::LambdaEvent::new(input, context);

    example_handler(event).await.expect("failed to fetch");
}
