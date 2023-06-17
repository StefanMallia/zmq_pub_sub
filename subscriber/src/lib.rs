use std::sync::Arc;

pub struct Subscriber
{
  socket: Arc<futures::lock::Mutex<zmq::Socket>>,
}

impl Subscriber
{
  pub fn new(channels: Vec<&str>, connection_string: &str, bind: bool) -> Subscriber
  {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    if bind
    {
      socket.bind(connection_string).unwrap();
      rust_log::info!("Subscriber listening on: {}", &connection_string)
    }
    else
    {
      socket.connect(connection_string).unwrap();
      rust_log::info!("Subscriber connected to: {}", &connection_string)
    }
    for channel in channels
    {
      let filter = channel.as_bytes();
      socket.set_subscribe(&filter).unwrap();
    }
    let socket = Arc::new(futures::lock::Mutex::new(socket));
    Subscriber{socket}
  }

  pub async fn receive(&self) -> String
  {    
    match self.receive_raw().await.split("ZMQTOPICEND").collect::<Vec<&str>>().last()
    {
      Some(message) => message.to_string(),
      None => "No message received".to_string()
    }
  }
  
  pub async fn receive_raw(&self) -> String
  {
    let message_result = self.socket.lock().await.recv_string(0);
    match message_result
    {
      Ok(msg) => msg.unwrap(),
      Err(x) => x.to_string()
    }
  }

  pub async fn subscribe(&self, channels: Vec<&str>)
  {
    let socket = self.socket.lock().await;
    for channel in channels
    {
      let filter = channel.as_bytes();
      socket.set_subscribe(&filter).unwrap();
    }
  }
}


