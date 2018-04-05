extern crate serde_json;
extern crate jsonway;

use message::serde_json::value::Value as Json;


pub struct Message {
    username: String,
    channel: String,
    text: String,
    icon_emoji: String,
}


impl Message {

    pub fn build<F: Fn(&mut Message)>(builder: F) -> Message {
        let mut message = Message::new();
        builder(&mut message);
        message
    }

    pub fn to_json(&self) -> Json {
        jsonway::object(|json| {
            json.set("username", &self.username);
            json.set("channel", &self.channel);
            json.set("text", &self.text);
            json.set("icon_emoji", &self.icon_emoji);
        }).unwrap()
    }

    pub fn set(&mut self, key: &str, value: &str) -> () {
        let val = String::from(value);
        match key {
            "username" => self.username = val,
            "channel" => self.channel = val,
            "text" => self.text = val,
            "icon_emoji" => self.icon_emoji = val,
            _ => panic!("Undefined key: {}", key)
        };
    }

    fn new() -> Message {
        Message {
            username: "slack-notify".to_string(),
            channel: "#general".to_string(),
            text: "This is a test message.".to_string(),
            icon_emoji: ":ghost:".to_string(),
        }
    }

}
