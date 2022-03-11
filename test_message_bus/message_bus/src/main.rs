#[tokio::main]
pub async fn main()
{
    println!("Hello, world!");
    let config_helper: config_helper::ConfigHelper = config_helper::ConfigHelper::new("appconfig.toml");
    let message_bus_pub_address = config_helper.get_value("message_bus_address_xpub").unwrap();
    let message_bus_sub_address = config_helper.get_value("message_bus_address_xsub").unwrap();
    let publisher = publisher::Publisher::new(message_bus_pub_address.as_str(), true);
    let subscriber = subscriber::Subscriber::new("", message_bus_sub_address.as_str(), true);
    
    while true
    {

      let message = subscriber.receive_raw().await;
      println!("{} Time received: {}", message, 
               chrono::Utc::now().timestamp_nanos());
      publisher.send_string("", &message.as_str());
    }

}
