#![allow(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
use clap::{
    Command,
    Arg,
    ArgMatches,
};
use tokio::net::{
    TcpListener,
    TcpStream,
};
use tokio::io;
use tokio::io::AsyncWriteExt;




fn setup_args() -> ArgMatches{
    
    let args = Command::new("Sidewinder")
    .version("1.0")
    .about("Forward Local and Remote Ports")
    .author("Ranger11Danger")
    .arg(Arg::new("forward")
        .short('L')
        .required(true)
        .require_delimiter(true)
        .help("Forward a Local Port to a Remote Address")
        .value_names(&["LOCALPORT", "REMOTEADDRESS", "REMOTEPORT"])
        .value_delimiter(':'))
    .get_matches();
    
    args
}


async fn start_tunnel(lport: String, address: String, rport: String) -> io::Result<()> {
    
    let listener = TcpListener::bind(format!("0.0.0.0:{lport}")).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        handle_connection(socket, &address, &rport).await;
    }

}

async fn handle_connection(mut in_stream: TcpStream, address: &str, rport: &str){
    
    let mut forward_stream = TcpStream::connect(format!("{address}:{rport}")).await.unwrap();

    let (mut r_in_stream, mut w_in_stream) = in_stream.split();
    let (mut r_forward_stream, mut w_forward_stream) = forward_stream.split();

    let client_to_server = async{
        io::copy(&mut r_in_stream, &mut w_forward_stream).await;
        w_forward_stream.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut r_forward_stream, &mut w_in_stream).await;
        w_in_stream.shutdown().await   
    };

    tokio::try_join!(client_to_server, server_to_client);
}

#[tokio::main]
async fn main()  {
    
    let my_args = setup_args();
    let arg_values: Vec<_> = my_args.values_of("forward").unwrap().collect();
    start_tunnel(arg_values[0].to_string(), arg_values[1].to_string(), arg_values[2].to_string()).await;

}
