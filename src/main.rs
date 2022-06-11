use aws_sdk_ec2::{model::InstanceStateName, Client, Error};

async fn show_state(client: &Client, ids: Option<Vec<String>>) -> Result<(), Error> {
    let resp = client
        .describe_instances()
        .set_instance_ids(ids)
        .send()
        .await?;

    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            let state = instance.state().unwrap().name().unwrap();
            match state {
                InstanceStateName::Running => {
                    let tags = instance.tags().unwrap();
                    for t in tags {
                        if t.key().unwrap() == "Name" {
                            println!("Name: {:?}", t.value().unwrap());
                        }
                    }
                    println!("Instance ID: {}", instance.instance_id().unwrap());
                    println!("State: {:?}", state);
                    println!();
                }
                _ => (),
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let region_provider = RegionProviderChain::first_try(region.map(Region::new));
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    // let instance_id: Option<Vec<String>> = Some(Vec::from(["i-02ac2100214309b9d".to_string()]));
    // show_state(&client, instance_id).await
    show_state(&client, None).await // list all
}
