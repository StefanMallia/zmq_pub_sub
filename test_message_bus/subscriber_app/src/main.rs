pub fn main()
{
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");
    let message_bus_sub_address = config_loader.get_string("message_bus_address_pub").unwrap();

    let subscriber = subscriber::Subscriber::new(vec!["Publisher"], message_bus_sub_address.as_str(), false);
    let mut num = 0;
    let mut publisher1num = 0;
    let mut sum_time_transit = 0;
    loop
    {
      let message = subscriber.receive();
      println!("Message received: {}", message);
      let message_split = message.split(" ").collect::<Vec<&str>>();
      let time_sent = message_split[2].parse::<i64>().unwrap();
      let time_received = chrono::Utc::now().timestamp_nanos();
      let message_number = message_split[3].parse::<i64>().unwrap();
      if message_split[1] == "1" && message_number != publisher1num
      {
          panic!("Expected message number {}, received {} instead. (Message: {})", num, message_number, message)
      }
      else if message_split[1] == "1" 
      {
        publisher1num = publisher1num + 1;          
      }
      num = num + 1;      
      sum_time_transit = sum_time_transit + time_received - time_sent;
      println!("Average time to receive {} ms", sum_time_transit as f64 / num as f64 / 1000.0 / 1000.0);

    }
}
