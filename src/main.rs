use device_query::{DeviceEvents, DeviceState};

fn main() {

    let device_state = DeviceState::new();
   
    let _guard = device_state.on_key_down(|key| {
       println!("{}", key.to_string());
    });
   
    loop {}
}