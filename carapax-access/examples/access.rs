use carapax::{handler, longpoll::LongPoll, types::Message, Api, Config, Dispatcher};
use carapax_access::{AccessHandler, AccessRule, InMemoryAccessPolicy};
use dotenv::dotenv;
use env_logger;
use std::env;

#[allow(clippy::trivially_copy_pass_by_ref)]
#[handler]
async fn handle_message(_context: &(), message: Message) {
    log::info!("Got a new message: {:?}", message);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let username = env::var("TGRS_ACCESS_USERNAME").expect("TGRS_ACCESS_USERNAME");

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).expect("Failed to create API");

    // Deny from all except for @username (specify without @)
    let rule = AccessRule::allow_user(username);
    let policy = InMemoryAccessPolicy::default().push_rule(rule);

    let mut dispatcher = Dispatcher::new(());
    dispatcher.add_handler(AccessHandler::new(policy));
    dispatcher.add_handler(handle_message);
    LongPoll::new(api, dispatcher).run().await
}
