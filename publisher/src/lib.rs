use std::sync::Arc;

pub struct Publisher
{
  socket: Arc<futures::lock::Mutex<zmq::Socket>>,
}

impl Publisher
{
  pub fn new(connection_string: &str, bind: bool) -> Publisher
  {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUB).unwrap();
    if bind
    {
      socket.bind(connection_string).unwrap();
    }
    else
    {
      socket.connect(connection_string).unwrap();
    }
    let socket = Arc::new(futures::lock::Mutex::new(socket));
    Publisher{socket}
  }

  pub fn send_string(&self, channel: &str, data: &str)
  {
    let message;
    if channel != ""
    {
      message = [channel,
                 "ZMQTOPICEND",                   
                  data].join("");
    }
    else
    {
      message = data.to_string();
    }
    
    tokio::spawn(
    {
      let socket = Arc::clone(&self.socket);
      async move
      {
        socket.lock().await.send(&message, 0).expect(format!("Failed to send: {}", &message).as_str());
      }
    });
  }

  pub fn send_bytes(&self, channel: &str, data: &[u8])
  {
    let message = [channel.as_bytes(),
                  "ZMQTOPICEND".as_bytes(),                   
                  data].concat();
    
    tokio::spawn(
    {
      let socket = Arc::clone(&self.socket);
      async move
      {
        socket.lock().await.send(&message, 0).expect(format!("Failed to send: {:?}", &message).as_str());
      }
    });
  }


  pub fn send_serializable_object<T>(&self, channel: &str, data: &T)
    where T: ?Sized + serde::Serialize
  {
    let message = [channel,
                   "ZMQTOPICEND",                   
                   &serde_json::to_string(data).unwrap().as_str()].join("");
    
    tokio::spawn(
    {
      let socket = Arc::clone(&self.socket);
      async move
      {
        socket.lock().await.send(&message, 0).expect(format!("Failed to send: {}", &message).as_str());
      }
    });
  }
}


