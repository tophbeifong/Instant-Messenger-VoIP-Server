use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader,BufWriter};
use std::fs::File;

fn load_config_files(){
    println!("Loading configuration file...");

    let mut settings_vector: Vec<&str> = Vec::new();

    //function returns a vector with the application settings...
    let settings = "config/settings.conf";
    let handle = File::open(settings).unwrap();

    for line in BufReader::new(handle).lines() {
        let mut line_data = &line.unwrap();

        //get the first charater, if its a # then the line is ignored... e.g. comments
        let first_character = &line_data[0..1];
        if first_character != "#"{
            //sort into a vector
            let mut settings_info_split = line_data.split("=");
            let settings_info = settings_info_split.collect::<Vec<&str>>();

            //need to add the values to the "settings_vector"
        }
    }
}

fn error_log(error_to_log: &str){
    //log the error which has been provided...

    //fetch error log filename from settings file... until then its hardcoded :/
    /*
    let log_file = "logs/error.log";
    let p = Path::new(log_file);

    //check the file was opened correctly. Thanks "stackoverflow"
    let mut f = match File::open_mode(&p, Open, Write) {
        Ok(f) => f,
        Err(e) => fail!("Unable to open error log: {}", e),
    };

    //write the error
    f.write_line("[ERROR DATE]: {}", error_to_log);
    */
    println!("ERROR");
}

fn get_active_user_ip(user_id: &str) -> &str{

    //just a place holder until i've finished fetch IP functionality
    let ip: &str = "192.168.1.139";

    return ip;

}

fn forward_message(ip_address_to_send: &str, message: &str){

    //build the full IP:PORT string for the socket
    //eventually the port will be gathered from a settings file. Until then hard coded.
    let port: &str = "7979";
    let full_address = format!("{}:{}", ip_address_to_send, port);

    println!("build ip/port string");

    //block just to create the socket and forward the message
    {
        //having trouble creating the socket to forward the data on...

        println!("Entering socket block");

        let mut forward_socket = TcpStream::connect(full_address).unwrap();

        println!("Socket Done");

        let _ = forward_socket.write(b"data to forward");

        println!("data send");

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

    println!("RAW DATA: {}", message_string);

    let message_sub_data_segments = message_string.split("][");

    let message_vector = message_sub_data_segments.collect::<Vec<&str>>();

    if message_vector[0] == "%^rpa)"{

        //forward the message onto the correct correspontand(s)
        //user MUST be online else the message will not be delivered. No messages will
        //be stored on the server. Everything is live or not at all!!!

        //find the correspontand ip address(They must be online else there is no ip address)

        //check it its a call or message... might aswell build for the future now
        if message_vector[1] == "message" {

            //get the target users ip address, need to forward the message some how...
            let ip_address: &str = get_active_user_ip(message_vector[2]);

            println!("Incomming data: {}", message_vector[5]);

            //forward the message to the target
            forward_message(ip_address, message_vector[5]);



        }else if message_vector[1] == "call"{

            //make call... NOT READY YET!
            //need go build all functionality for calls
            //calls will work similar to how they work on discord... Only send data when the user speaks.


        }

    }else{

        //invalid message recieved. log the anomaly
        error_log("Invalid connection attempt.");

    }
}

fn main(){
    //Need to open a TCP server and listen... then we parse the relevant data from the transmission and
    //relocate it to the correct correspontand
    println!("IMServer is starting up...");
    load_config_files();

    println!("Building TCP server...");
    let tcp_listener = TcpListener::bind("192.168.1.215:7979").unwrap();

    //println!("TCP server established, listening on port {}", );
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
                //build the error message and pass it to the error_log function
                let connection_error_string = format!("Listener creation failed: {}", connection_error);

                error_log(connection_error_string.as_str());
            }

        }

    }

}
