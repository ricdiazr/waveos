use super::ComponentError as OSCComponentError;
use super::OSCContainer;
use super::OSCCommand;
use std::collections::HashMap;
use Mutex;
use Arc;
/// Component: Command Broker
#[inline]
pub fn accept(
    command: OSCCommand,
    address_space: Arc<Mutex<HashMap<String, OSCContainer>>>,
) -> Result<(), OSCComponentError> {
    let space = address_space
        .lock()
        .map_err(|_| OSCComponentError::BrokerError("Mutex lock failed".to_string()))?;
    println!("Delivering command '{}' using address space: {:?}", command, *space);
    Ok(())
}
