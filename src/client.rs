use std::ffi::c_float;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;
use vigem::*;
use vigem_client::XButtons;

//scan for open 14987 port
fn scan(port: u16) -> UdpSocket {
    // addrs will str be 0.0.0.0:port
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port).to_string();

    let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");
    let mut buf = [0u8; 1024];
    //send broadcast message to 14987 port and wait for response
    let port = port.clone().to_string();
    let port = port.as_bytes();
    socket
        .send_to(port, "255.255.255.255:14987")
        .expect("couldn't send message");
    //wait for response
    socket.recv_from(&mut buf).expect("didn't receive data");
    println!("sent bytes");
    socket
}

fn axis_changed(
    axis: &str,
    value: &str,
    controller: &mut Target,
    gamepad: &mut XUSBReport,
    vigem: &mut Vigem,
) {
    let value = value.trim();

    let value: f32 = match value.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("value is not valid");
            return;
        }
    };

    match axis {
        "LeftStickX" => gamepad.s_thumb_lx = (value * 30000.0) as i16,
        "LeftStickY" => gamepad.s_thumb_ly = (value * 30000.0) as i16,
        "RightStickX" => gamepad.s_thumb_rx = (value * 30000.0) as i16,
        "RightStickY" => gamepad.s_thumb_ry = -((value * 30000.0) as i16),
        "LeftTrigger" => gamepad.b_left_trigger = (value * 255.0) as u8,
        "RightTrigger" => gamepad.b_right_trigger = (-value * 255.0) as u8,
        _ => return,
    }
    let _ = vigem.update(controller, gamepad);
}

fn main() {
    //get port from command line
    let args: Vec<String> = std::env::args().collect();
    //if no port is provided, set to 9761
    let port = if args.len() < 2 {
        "9761".to_string()
    } else {
        args[1].clone()
    };
    println!("port: {}", port);
    //if port is not valid, exit
    let port: u16 = match port.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("port is not valid");
            return;
        }
    };
    if !(1024..=65535).contains(&port) {
        println!("port is not valid");
        return;
    }

    let client = vigem_client::Client::connect();
    //if ok, start the controller else exit and ask to install ViGEmBus
    let _client = match client {
        Ok(client) => client,
        Err(_) => {
            println!("ViGEmBus is not installed");
            return;
        }
    };
    let socket = scan(port);
    println!("found server at: {}", socket.local_addr().unwrap());
    let mut vigem = Vigem::new();
    match vigem.connect() {
        Ok(_) => {}
        Err(_) => {
            println!("ViGEmBus is not installed");
            return;
        }
    }
    let id = TargetType::Xbox360;
    let mut controller = Target::new(id);
    vigem.target_add(&mut controller).unwrap();
    // Now it's connected!
    dbg!(controller.state());
    let mut gamepad = XUSBReport {
        w_buttons: XButton::Nothing,
        ..XUSBReport::default()
    };
    loop {
        let mut buf = [0u8; 1024];

        let (_amt, _src) = socket.recv_from(&mut buf).expect("didn't receive data");
        //decode message
        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("received message: {}", s);
        let _controler_id = vigem_client::TargetId::XBOX360_WIRED;

        //end the string at **, so we can parse it
        let s = s.split("**").collect::<Vec<&str>>();
        let s2 = s[0];
        let s = s[0];
        //parse the message : EventType(event_name, event_value ,  Code(EvCode(EvCode { kind: Axis, index: 2 })))*id** garbage
        let s: Vec<&str> = s.split('(').collect();
        let event_type = s[0];
        let s: Vec<&str> = s[1].split(',').collect();
        let mut event_name = s[0];
        let event_value = s[1];
        //split for * so we can get the id of the controller
        let s: Vec<&str> = s2.split('*').collect();
        let id = s[1];
        //get kind and index
        let s: Vec<&str> = s2.split("kind:").collect();
        let s: Vec<&str> = s[1].split(',').collect();
        let kind = s[0].trim();
        let s: Vec<&str> = s2.split("index:").collect();
        let s: Vec<&str> = s[1].split('}').collect();
        let index = s[0].trim();
        //if kind == Axis, and index == 2 then it's RightStickX
        if kind == "Axis" && index == "2" {
            event_name = "RightStickX";
        }
        //if kind == Axis, and index == 3 then it's RightStickY
        if kind == "Axis" && index == "3" {
            event_name = "RightStickY";
        }

        //if kind == Axis, and index == 4 or 5 then it's LeftTrigger or RightTrigger
        if kind == "Axis" && index == "4" {
            event_name = "RightTrigger";
        }
        if kind == "Axis" && index == "5" {
            event_name = "LeftTrigger";
        }

        println!(
            "EventType: {}, event_name: {}, event_value: {}, id: {}",
            event_type, event_name, event_value, id
        );

        match event_type {
            "AxisChanged" => axis_changed(
                event_name,
                event_value,
                &mut controller,
                &mut gamepad,
                &mut vigem,
            ),
            "ButtonChanged" => button_changed(
                event_name,
                event_value,
                &mut controller,
                &mut gamepad,
                &mut vigem,
            ),
            _default => {
                println!("EventType is not valid");
            }
        }
    }
}

fn button_changed(
    button: &str,
    value: &str,
    controller: &mut Target,
    gamepad: &mut XUSBReport,
    vigem: &mut Vigem,
) {
    //trim value and parse it to float
    let value = value.trim();
    let value: c_float = match value.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("value is not valid");
            return;
        }
    };

    //if value is 1, set the button to true, else set it to false
    let _b_value = value == 1.0;
    //bit mask for the buttons 32768 max
    let reset_mask;
    let button_mask;
    let _left_trigger2 = 0;
    let _right_trigger2 = 0;
    //if value is 1 we need to set the button to true so button_mask will have corresponding bit set to 1
    // if value is 0 we need to set the button to false so reset_mask will have corresponding bit set to 0
    if value > 0.0 {
        //set the corresponding bit to 1
        match button {
            "East" => button_mask = XButtons::A,
            "South" => button_mask = XButtons::B,
            "West" => button_mask = XButtons::Y,
            "North" => button_mask = XButtons::X,
            "Start" => button_mask = XButtons::START,
            "Select" => button_mask = XButtons::BACK,
            "DPadRight" => button_mask = XButtons::RIGHT,
            "DPadLeft" => button_mask = XButtons::LEFT,
            "DPadUp" => button_mask = XButtons::UP,
            "DPadDown" => button_mask = XButtons::DOWN,
            "LeftThumb" => button_mask = XButtons::LTHUMB,
            "RightThumb" => button_mask = XButtons::RTHUMB,
            "RightTrigger" => button_mask = XButtons::RB,
            "LeftTrigger" => button_mask = XButtons::LB,
            _ => return,
        }
        gamepad.w_buttons = XButton::from_bits(gamepad.w_buttons.bits() | button_mask).unwrap();
    } else {
        match button {
            "East" => reset_mask = !XButtons::A,
            "South" => reset_mask = !XButtons::B,
            "West" => reset_mask = !XButtons::Y,
            "North" => reset_mask = !XButtons::X,
            "Start" => reset_mask = !XButtons::START,
            "Select" => reset_mask = !XButtons::BACK,
            "DPadRight" => reset_mask = !XButtons::RIGHT,
            "DPadLeft" => reset_mask = !XButtons::LEFT,
            "DPadUp" => reset_mask = !XButtons::UP,
            "DPadDown" => reset_mask = !XButtons::DOWN,
            "LeftThumb" => reset_mask = !XButtons::LTHUMB,
            "RightThumb" => reset_mask = !XButtons::RTHUMB,
            "RightTrigger" => reset_mask = !XButtons::RB,
            "LeftTrigger" => reset_mask = !XButtons::LB,
            _ => return,
        }
        gamepad.w_buttons = XButton::from_bits(gamepad.w_buttons.bits() & reset_mask).unwrap();
    }
    vigem.update(controller, gamepad).unwrap();
}
