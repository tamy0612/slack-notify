extern crate slack_notify;

#[macro_use]
extern crate clap;
use clap::{App, Arg, AppSettings};


fn channel_validation(v: String) -> Result<(), String> {
    if !(v.starts_with("@") || v.starts_with("#")) {
        return Err(String::from("post target should start with '@' or '#'"))
    }
    return Ok(())
}


fn main() {
    let matches = App::new("slack-notify")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::from_usage("<workspace> -w --workspace [WORKSPACE] 'webhook URL for workspace (Required)'"))
        .arg(Arg::from_usage("[target] -p --post-to [TARGET] 'post target")
             .default_value("#general")
             .validator(channel_validation))
        .arg(Arg::from_usage("[username] -u --username [USERNAME] 'username of this webhook'")
             .default_value("slack-notification"))
        .arg(Arg::from_usage("[icon] -i --icon [ICON] 'icon string to display itself'")
             .default_value(":ghost:"))
        .arg(Arg::with_name("body").help("message body"))
        .get_matches();
    let message = slack_notify::Message::build(|message| {
        if let Some(target) = matches.value_of("target") {
            message.set("channel", target);
        }
        if let Some(username) = matches.value_of("username") {
            message.set("username", username);
        }
        if let Some(icon) = matches.value_of("icon") {
            message.set("icon_emoji", icon);
        }
        if let Some(text) = matches.value_of("text") {
            message.set("text", text);
        }
    });
    match slack_notify::post(matches.value_of("workspace").unwrap().to_string(), &message) {
        Ok(()) => {},
        Err(error) => panic!("Error: {}", error)
    };
}
