#[tokio::main]
pub async fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");
    let message_bus_pub_address = config_loader.get_string("message_bus_address_pub").unwrap();
    let message_bus_sub_address = config_loader.get_string("message_bus_address_sub").unwrap();
    let publisher = publisher::Publisher::new(message_bus_pub_address.as_str(), true);
    let subscriber = subscriber::Subscriber::new(vec![""], message_bus_sub_address.as_str(), true);
    
    loop
    {

      let message = subscriber.receive_raw().await;
      println!("{} Time received: {}", message, 
               chrono::Utc::now().timestamp_nanos());
      publisher.send_string("", &message.as_str());
    }

}
