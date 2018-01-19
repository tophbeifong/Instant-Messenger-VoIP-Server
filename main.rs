use std::net::TcpListener;
use std::thread;
use std::io::{BufReader,BufWriter};
use std::io::Write;
use std::io::Read;

//fn error_log(string: error_message){

//}

fn get_active_user_ip(mut user_id: &str){

    let mut ip = "192.168.1.32";

}

fn forward_message(mut ip_address: String, mut message: String){

    //build the full IP:PORT string for the socket
    //no doubt there is an error somewhere in here...
    let port = "7979";

    let colon = ":";

    ip_address.push_str(&colon);

    ip_address.push_str(&port);

    {
        println!("Forwarding message: {} - {}", message, ip_address);
        //create a socket to send forward the message onto the correct destination
        //let mut send_stream = TcpStream::connect(full_address).unwrap();

        //write the message via the socket
        //send_stream.write(message);

        //close the socket
    }

}

fn process_manager(stream: std::net::TcpStream){

    //extract the sub data from the encoded message
    //message structure as follows

    //key][data_type][correspondantID,correspondantID][date/time][platform][message
    //messages should be in base64. Its upto client devs to encrypt messages...

    //sort the data from the TCPStream
    let mut reader = BufReader::new(&stream);
    let mut message_string = String::new();
    reader.read_to_string(&mut message_string).expect("could not read");

    //first we chack the message has a valid tag else it will be discarded
    //split the message and store it into a vector

    let mut message_sub_data_segments = message_string.split("][");

    let message_vector = message_sub_data_segments.collect::<Vec<&str>>();

    if message_vector[0] == "%^rpa)"{

        //forward the message onto the correct correspontand(s)
        //user MUST be online else the message will not be delivered. No messages will
        //be stored on the server. Everything is live or not at all!!!

        //find the correspontand ip address(They must be online else there is no ip address)

        //check it its a call or message... might aswell build for the future now
        if message_vector[1] == "message" {

            //get the target users ip address, need to forward the message some how...
            let mut ip_address: String = get_active_user_ip(message_vector[2]);

            ip_address = "192.168.1.32";

            //forward the message to the target
            forward_message(ip_address, message_vector[5].to_string());



        }else if message_vector[1] == "call"{

            //make call... NOT READY YET!
            //need go build all functionality for calls

        }



    }else{

        //invalid message recieved. log the anomaly
        //error_log("Invalid message type recieved.");

    }
}

fn main(){
    //Need to open a TCP server and listen... then we parse the relevant data from the transmission and
    //relocate it to the correct correspontand

    let tcp_listener = TcpListener::bind("127.0.0.1:7979").unwrap();

    //listen for incomming connections
    for stream in tcp_listener.incoming(){

        match stream {

            Ok(data) => {
                //spawn a new thread to handle each incomming message
                thread::spawn(move ||{

                    //pass the data over to the process manager
                    process_manager(data);

                }).join();

            }

            //handle failed connections
            Err(connection_error) => {
                //error_log("Attemted connection unsuccessful.");
            }

        }

    }

}