pub struct Subscriber
{
  socket: zmq::Socket,
}

impl Subscriber
{
  pub fn new(channels: Vec<&str>, connection_string: &str, bind: bool) -> Subscriber
  {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    for channel in channels
    {
      let filter = channel.as_bytes();
      socket.set_subscribe(&filter).unwrap();
    }
    if bind
    {
      socket.bind(connection_string).unwrap();
    }
    else
    {      
      socket.connect(connection_string).unwrap();
      rust_log::info!("Subscriber connected to: {}", &connection_string)
    }
    Subscriber{socket}
  }

  pub fn receive(&self) -> String
  {    
    match self.receive_raw().split("ZMQTOPICEND").collect::<Vec<&str>>().last()
    {
      Some(message) => message.to_string(),
      None => "No message received".to_string()
    }
  }
  
  pub fn receive_raw(&self) -> String
  {
    let message_result = self.socket.recv_string(0);
    match message_result
    {
      Ok(msg) => msg.unwrap(),
      Err(x) => x.to_string()
    }
  }

  pub fn subscribe(&self, channels: Vec<&str>)
  {
    for channel in channels
    {
      let filter = channel.as_bytes();
      self.socket.set_subscribe(&filter).unwrap();
    }
  }
}

