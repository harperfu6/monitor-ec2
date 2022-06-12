use aws_sdk_ec2::{model::InstanceStateName, model::Tag, Client, Error};
use curl::easy::Easy;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::http_client::default_client;
use slack_rust as slack;
use std::env;
use std::fmt::{Display, Write};
use std::io::Read;

// Monitoring Instance
#[derive(Debug)]
struct MInstance {
    instance_id: String,
    name: String,
    key_name: String,
    state: InstanceStateName,
}

#[allow(unused_must_use)]
impl Display for MInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "instance_id: {}\n", self.instance_id);
        write!(f, "Name: {}\n", self.name);
        write!(f, "Key Name: {}\n", self.key_name);
        write!(f, "Status: {:?}\n", self.state);
        f.write_char('\n');
        Ok(())
    }
}

async fn show_state(client: &Client, ids: Option<Vec<String>>) -> Result<Vec<MInstance>, Error> {
    let resp = client
        .describe_instances()
        .set_instance_ids(ids)
        .send()
        .await?;

    let mut m_instance: Vec<MInstance> = Vec::new();
    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            let state = instance.state().unwrap().name().unwrap();
            match state {
                InstanceStateName::Running => {
                    let instance_id = instance.instance_id().unwrap();
                    println!("Instance ID: {}", instance_id);
                    let key_name = instance.key_name().unwrap();

                    let tags = instance.tags().unwrap();
                    if let Some(name_tag) =
                        tags.iter().filter(|t| t.key().unwrap() == "Name").next()
                    {
                        let name_value = name_tag.value().unwrap().to_string();
                        println!("Name: {:?}", name_value);
                        m_instance.push(MInstance {
                            instance_id: instance_id.to_string(),
                            name: name_value,
                            key_name: key_name.to_string(),
                            state: state.clone(),
                        });
                    } else {
                        m_instance.push(MInstance {
                            instance_id: instance_id.to_string(),
                            name: "No-Name".to_string(),
                            key_name: key_name.to_string(),
                            state: state.clone(),
                        });
                    }
                    println!("Key Name: {}", key_name);
                    println!("State: {:?}", state);
                    println!();
                }
                _ => (),
            }
        }
    }

    Ok(m_instance)
}

// incomming web hook
#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let m_instance_vec: Vec<MInstance> = show_state(&client, None).await?; // list all
    let message: String = m_instance_vec.iter().map(|mi| mi.to_string()).collect();

    let message_json_str: String = format!("{{\"text\":\"{}\"}}", message);
    let mut data = message_json_str.as_bytes();

    let webhook_url = env::var("SLACK_CHANNEL_WEBHOOK_URL")
        .unwrap_or_else(|_| panic!("SLACK_CHANNEL_WEBHOOK_URL is not set."));
    let mut easy = Easy::new();
    easy.url(&webhook_url).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer.perform().unwrap();

    Ok(())
}

// api token
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let shared_config = aws_config::load_from_env().await;
//     let client = Client::new(&shared_config);

//     let m_instance_vec: Vec<MInstance> = show_state(&client, None).await?; // list all
//     let message: String = m_instance_vec.iter().map(|mi| mi.to_string()).collect();

//     let slack_bot_token =
//         env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));

//     let slack_api_client = default_client();
//     let bot_test_channel_id = "C02ML9PTSJD";
//     let param = PostMessageRequest {
//         channel: bot_test_channel_id.to_string(),
//         text: Some(message),
//         ..Default::default()
//     };

//     let response = post_message(&slack_api_client, &param, &slack_bot_token)
//         .await
//         .expect("api call error");
//     println!("{:?}", response);

//     Ok(())
// }
