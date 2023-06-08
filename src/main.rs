use gilrs::{Button, Event, Gilrs};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

//fn to init socket
fn init_socket() -> UdpSocket {
    let socket = UdpSocket::bind("0.0.0.0:14987").expect("couldn't bind to address");
    return socket;
}

fn main() {
    println!("Hello, world!");
    let socket = init_socket();
    let mut gilrs = Gilrs::new().unwrap();

    //wait for a client to connect, then we will send data to it
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).expect("Didn't receive data");
    let ip = src.ip();
    //read data from client, that will be the port number

    let port = String::from_utf8(buf[..amt].to_vec()).unwrap();
    //respond to client
    let data = format!("Hello, {:?}!", src);
    println!("Received data from: {}:{}", ip, port);
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            //send data to client
            let mut data = format!("{:?}*", event);
            data += id.to_string().as_str();
            data += "**";

            socket
                .send_to(data.as_bytes(), src)
                .expect("couldn't send data");
        }
    }
}
