pub fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new("appconfig.toml");
    let message_bus_pub_address = config_loader.get_string("message_bus_address_sub").unwrap();

    let publisher = publisher::Publisher::new(message_bus_pub_address.as_str(), false);
    let mut num: i64 = 0;
    while true
    {
      println!("sending message {}", num);
      
      let channel = ["Publisher", args[1].as_str()].join("");
      let message = [
        "Publisher", args[1].as_str(),
         chrono::Utc::now().timestamp_nanos().to_string().as_str(),
        num.to_string().as_str()
        ].join(" ");
        
      publisher.send_string(&channel, &message);
      std::thread::sleep(std::time::Duration::from_millis(100));
      num += 1;
    }
}
