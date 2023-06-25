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
      //ensure_subscriber_binding_startup_finished(ctx, connection_string, &socket);      
    }
    else
    {      
      socket.connect(connection_string).unwrap();
      std::thread::sleep(std::time::Duration::from_millis(500));
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



fn ensure_subscriber_binding_startup_finished(ctx: zmq::Context, connection_string: &str, socket: &zmq::Socket) {
  let pub_socket = ctx.socket(zmq::PUB).unwrap();
  pub_socket.connect(connection_string).unwrap();
  let cancellation_token = std::sync::Arc::new(std::sync::RwLock::new(false));
  let cancellation_token_clone = cancellation_token.clone();

  std::thread::spawn(
    move ||
    {
      while *cancellation_token_clone.read().unwrap() == false
      {        
        pub_socket.send("STARTUP_CONFIRMATION_CHANNEL", 0).unwrap();
      }
    }
  );

  socket.set_subscribe("STARTUP_CONFIRMATION_CHANNEL".as_bytes()).unwrap();
  socket.recv_string(0).unwrap().unwrap();  
  *cancellation_token.write().unwrap() = true;
  socket.set_unsubscribe("STARTUP_CONFIRMATION_CHANNEL".as_bytes()).unwrap();

  rust_log::info!("Subscriber listening on: {}", &connection_string)
}

