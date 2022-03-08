#[tokio::main]
pub async fn main()
{
    let config_helper: config_helper::ConfigHelper = config_helper::ConfigHelper::new("appconfig.toml");
    let message_bus_sub_address = config_helper.get_value("message_bus_address_xpub").unwrap();

    let subscriber = subscriber::Subscriber::new("Publisher", message_bus_sub_address.as_str(), false);
    loop
    {
      let message = subscriber.receive().await;
      println!("Message received: {}", message);
    }
}
