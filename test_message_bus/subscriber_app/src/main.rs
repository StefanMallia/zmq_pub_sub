#[tokio::main]
pub async fn main()
{
    let config_helper: config_helper::ConfigHelper = config_helper::ConfigHelper::new("appconfig.toml");
    let message_bus_sub_address = config_helper.get_value("message_bus_address_xpub").unwrap();

    let subscriber = subscriber::Subscriber::new("Publisher", message_bus_sub_address.as_str(), false);
    let mut num = 0;
    let mut sum_time_transit = 0;
    loop
    {
      let message = subscriber.receive().await;
      println!("Message received: {}", message);
      let message_split = message.split(" ").collect::<Vec<&str>>();
      let time_sent = message_split[2].parse::<i64>().unwrap();
      let time_received = chrono::Utc::now().timestamp_nanos();
      num = num + 1;
      sum_time_transit = sum_time_transit + time_received - time_sent;
      println!("Average time to receive {}", sum_time_transit as f64 / num as f64 / 1000.0 / 1000.0);

    }
}
