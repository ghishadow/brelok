use clap::Parser;
use owo_colors::{OwoColorize, Style};
use log::{info, trace, warn};
use ssh2::Session;

use nix::unistd::{Uid, User};

// #[derive(Parser, Debug)]
// #[clap(name = "brelok")]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[clap(subcommand)]
//     agents: Commands,
// }

// #[derive(Debug, SubCommand)]
// enum Commands {

// }

pub(crate) fn get_system_user(name: &str) -> anyhow::Result<User> {
    if Uid::current().is_root() {
        Ok(User::from_name(name)
            .with_context(|| format!("could not query {} user information", name))?
            .ok_or_else(|| anyhow!("could not query {} user information", name))?)
    } else if cfg!(debug_assertions) {
        Ok(User::from_uid(Uid::current())
            .context("could not query current user information")?
            .ok_or_else(|| anyhow!("could not query current user information"))?)
    } else {
        Err(anyhow!("this command must be run as root"))
    }
}


struct Brelok;

// impl Brelok {
//     fn get_user() {
//      unimplemented!("get_user");    
//     }
//     fn get_os() {
//      unimplemented!("get_os");

//     }
//     fn verify_key_dir() {

//     }

//     fn lock_file() {

//     }

//     fn take_lock() {

//     }
    
//     fn drop_lock(){}


//     fn find_pids(){

//     }

//     fn stop_agent() {

//     }
    
//     fn inherit_agents() {

//     }

//     fn valid_inherit() {

//     }

//     fn catpidf_shell(){

//     }

//     fn catpidf(){

//     }

//     fn load_agents(){

//     }

//     fn start_agent() {

//     }


//     fn extract_fingerprints() {

//     }

//     fn ssh_l() {
        
//     }


//     fn ssh_f() {
        
//     }

//     fn ssh_listmissing()  {
        
//     }

//     fn add_ssh_key(){
        
//     }

//     fn parse_mykeys() {
        
//     }

//     fn set_action() {

//     }

//     fn set_agents() {
        
//     }

//     fn conf_path(){
        
//     }

//     fn want_agent(){
        
//     }
    


// }




fn main() {
    let text_style = Style::new().purple().bold();
    let info = "Brelok - a tiny ssh agent frontend"; 
    println!("{}", info.style(text_style));
    println!("{}", get_system_user("ghishadow"));
    // let args = Args::parse();

     // Almost all APIs require a `Session` to be available
    let sess = Session::new().unwrap();
    // match args.command {
        // Commands::Agents {} => {
// Connect the agent and request a list of identities
    let mut agent = sess.agent().unwrap();

    agent.connect().unwrap();
    agent.list_identities().unwrap();

for identity in agent.identities().unwrap() {
    println!("{}", identity.comment());
    let pubkey = identity.blob();

}
        // }
    // }

    println!("My number is {:#x}!", 10.green());
}
