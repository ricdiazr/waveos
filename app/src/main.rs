extern crate osc;
use crate::osc::broker as osc_broker;
use crate::osc::receiver as osc_receiver;
use crate::osc::receiver::PacketDataParseError;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::env;

// External Component wosc-app - simulattor -
fn _app_send( args: &Vec<String>) -> Result<osc::PacketData,PacketDataParseError> {

    // Check if a string argument was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <your_string_argument>", args[0]);
        std::process::exit(1);
    }

    // Retrieve the first argument (excluding the program name)
    let input = &args[1];

    // Print or process the input string
    println!("Received argument: {}", input);
    Ok(osc_receiver::parse_binary_to_packet(string_to_binary(input))?)
}

pub fn string_to_binary(input: &str) -> Vec<u8> {
    let content = input.as_bytes(); // Convert string to UTF-8 bytes
    let size = content.len() as u8; // Calculate the size (ensure it's <= 255)
    let mut binary_data = vec![size]; // Start with the size as the first byte
    binary_data.extend_from_slice(content); // Append the string's bytes
    binary_data
}

fn main() {
    // Shared resources
    let command_queue = Arc::new(Mutex::new(VecDeque::new()));
    let address_space = Arc::new(Mutex::new(HashMap::new()));

        // Address Space and Sign-in operations
    osc::sign_in("osc_container".to_string(), address_space.clone()).unwrap();
    osc::address_space("osc_container".to_string());

    // Simulate processing pipeline
    match _app_send(&env::args().collect()) {
        Ok(osc_packet) => {
          println!("Parsed PacketData: {:?}", osc_packet);
          if let Ok(osc_message) = osc_receiver::accept(osc_packet) {
            let queue = Arc::clone(&command_queue);
            let addr_space = Arc::clone(&address_space);

        // Scheduler processes received message
            osc::scheduler(osc_message.clone(), queue).unwrap();

        // Command Process Bus retrieves a command
            if let Some(command) = osc::command_process_bus(command_queue.clone()) {
              osc::monitor(&command);

            // Simulate address space and command broker processing
              osc_broker::accept(command, addr_space).unwrap();
            }
        }
      },
        Err(error) => println!("Failed to parse packet: {:?}", error),
    }
}
