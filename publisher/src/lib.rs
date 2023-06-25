use std::{sync::Arc, sync::Mutex, collections::VecDeque};

pub struct Publisher
{
  queue: Arc<Mutex<VecDeque<Box<[u8]>>>>
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
      let ctx = zmq::Context::new();
      //ensure_publisher_binding_startup_finished(ctx, connection_string, &socket);
    }
    else
    {      
      socket.connect(connection_string).unwrap();      
      std::thread::sleep(std::time::Duration::from_millis(500));
      rust_log::info!("Publisher connected to: {}", &connection_string)
    }    
    let queue = Arc::new(Mutex::new(VecDeque::<Box<[u8]>>::new()));
    let publisher = Publisher{queue: queue.clone()};
    std::thread::spawn(
      ||
      {
        Publisher::dequeue_and_send_loop(socket, queue)
      }      
    );
    publisher 
  }


  pub fn dequeue_and_send_loop(socket: zmq::Socket, queue: Arc<Mutex<VecDeque<Box<[u8]>>>>)
  {
    loop 
    {      
      let message_result = queue.lock().unwrap().pop_front();      
      match message_result
      {
        Some(message) => socket.send(&message, 0).unwrap(),
        None => ()
      }      
    }      
  }


  pub fn send_string(&self, channel: &str, data: &str)
  {
    let message = format_for_zmq(channel, data);
    self.queue.lock().unwrap().push_back(message);
  }

  pub fn send_bytes(&self, channel: &str, data: &[u8])
  {
    let message = format_bytes_for_zmq(channel, data);
    self.queue.lock().unwrap().push_back(message);    
  }


  pub fn send_json_serialized<T>(&self, channel: &str, data: &T)
    where T: ?Sized + serde::Serialize
  {
    let message = format_for_zmq(channel, &serde_json::to_string(data).unwrap().as_str());    
    self.queue.lock().unwrap().push_back(message);
  }
}

fn ensure_publisher_binding_startup_finished(ctx: zmq::Context, connection_string: &str, socket: &zmq::Socket) {
    let sub_socket = ctx.socket(zmq::SUB).unwrap();
    sub_socket.connect(connection_string).unwrap();
    let subscriber_socket_thread = std::thread::spawn(
      move ||
      {
          sub_socket.set_subscribe("STARTUP_CONFIRMATION_CHANNEL".as_bytes()).unwrap();            
          sub_socket.recv_string(0).unwrap().unwrap();            
      }
    );
    while !subscriber_socket_thread.is_finished()
    {
      socket.send("STARTUP_CONFIRMATION_CHANNEL", 0).unwrap();
    }
    rust_log::info!("Publisher listening on: {}", &connection_string)
}


fn format_for_zmq<'a>(channel: &'a str, data:  &'a str) -> Box<[u8]>
{
  if channel != ""
  {
    [channel.as_bytes(), "ZMQTOPICEND".as_bytes(), data.as_bytes()].concat().into_boxed_slice()
  }
  else
  {
    // Assume channel is already in data.
    data.as_bytes().to_vec().into_boxed_slice()
  }
}

fn format_bytes_for_zmq(channel: &str, data: &[u8]) -> Box<[u8]>
{  
  if channel != ""
  {
    [channel.as_bytes(), "ZMQTOPICEND".as_bytes(), data].concat().into_boxed_slice()
  }
  else
  {
    // Assume channel is already in data.
    data.to_vec().into_boxed_slice()
  }
}
