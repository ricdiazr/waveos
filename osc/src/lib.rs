use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::borrow::{Cow};

// Define types
//
//poor man's atomic types
//todo create atomic types based on AtomicPtr
pub type AtomicString = Arc<String>; //poor man's Atomic read only String
pub type AtomicBlob<'a> = Arc<Cow<'a,[u8]>>;
pub type AtomicChar = Arc<char>;
pub type AtomicU32 = Arc<u32>;

#[derive(Debug, Clone, Copy)]
struct FixedPointTime {
    integer: u32,   // High 32 bits (seconds)
    fractional: u32 // Low 32 bits (fractional seconds)
}

#[derive(Debug, Clone)]
pub struct AddrPattern(String);

#[derive (Clone, Debug)]
pub enum TypedData<'a> {
  Int32(AtomicU32),
  Float32(AtomicU32),
  Asciistring(AtomicString),
  Blob(AtomicBlob<'a>),
  Time(FixedPointTime),
  Asciichar(AtomicChar),
}

#[derive(Debug,Clone)]
pub enum Packet<'a> {
  Message { 
    address: AddrPattern,
    arguments: &'a [TypedData<'a>]
  },
  Bundle {
    timetag: TypedData<'a>::Time,
    elements: &'a [Packet<'a>],
  },
}
 
pub type Arguments = AtomicString;

pub type OSCContainer = String;
pub type OSCCommand = String;

#[derive (Debug)]
pub enum Failure {
    ReceiverError(String),
    SignInError(String),
    SchedulerError(String),
    BrokerError(String),
}

#[derive (Clone, Debug)]
pub struct MessageNode<'a> {
  packet: Packet<'a>,
}

impl MessageNode<'_> {

  const TYPE_TAG_PREFIX: char = ',';
  const ADDR_PATTERN_CONTAINER_SEP: char = '/';

  pub fn addr_pattern(&self) -> Option<AddrPattern> {
    match &self.packet {
//      Packet::Message { address,arguments } =>  Some(AddrPattern(Self::binary_to_utf8_str(address.clone()))),
      Packet::Message { address, .. } =>  Some(address.clone()),
        _ => None
      }
  }

  fn binary_to_utf8_str(data: Vec<u8>) -> String {
    String::from_utf8(data).expect("CONVERT from binary")
  }
}

/// Component: Sign In Controller
pub fn sign_in(
    container: OSCContainer, 
    address_space: Arc<Mutex<HashMap<String, OSCContainer>>>) -> Result<(), Failure> {
    println!("Registered container: {}", container);
    address_space
        .lock()
        .map_err(|_| Failure::SignInError("Mutex lock failed".to_string()))?
        .insert(container.clone(), container);
    Ok(())
}

/// Component: Scheduler
pub fn scheduler(message: MessageNode, queue: Arc<Mutex<VecDeque<OSCCommand>>>) -> Result<(), Failure> {
    let command = format!("processed_{:#?}", message);

    println!("got message address: {:#?}", message.addr_pattern().unwrap());

    queue
        .lock()
        .map_err(|_| Failure::SchedulerError("Mutex lock failed".to_string()))?
        .push_back(command.clone());
    println!("Scheduled command: {}", command);
    Ok(())
}

/// Component: Command Process Bus
pub fn command_process_bus(queue: Arc<Mutex<VecDeque<OSCCommand>>>) -> Option<OSCCommand> {
    queue.lock().ok()?.pop_front()
}

/// Component: Monitor
pub fn monitor(command: &OSCCommand) {
    println!("Monitoring command: {}", command);
}

/// Component: Address Space
/// registers 
#[inline]
pub fn address_space(container: OSCContainer) {
    println!("Address space updated for container: {}", container);
}

pub mod broker;
pub mod receiver;
