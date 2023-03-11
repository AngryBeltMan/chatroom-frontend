use reqwasm;
use gloo::console::log;
use serde::*;
pub type DynResult<T> = Result<T,Box<dyn std::error::Error>>;
#[derive(Debug,Deserialize,Serialize)]
struct Messages {
    messages:Vec<Message>
}
#[derive(Debug,Deserialize,Serialize)]
struct Message {
    messages:Vec<String>
}
#[derive(Serialize,Deserialize)]
struct Array ( Vec<String>);

const BACKENDURL:&str = "https://chatroom-backend-x12y.onrender.com";

pub async fn is_valid_chatroom(join_code:&str) -> bool {
    let url = format!("{BACKENDURL}/chatroom/valid/{join_code}");
    let get = reqwasm::http::Request::get(&url);
    match get.send().await {
        Ok(o) => {
            log!(format!("url {}",o.url()));
            let outcome = o.text().await.unwrap();
            if  outcome == String::from(r#""true""#) {
                log!("true");
                return true;
            } else {
                log!("false");
                return false;
            }
        },
        Err(_) => {
            return false;
        }
    }
}

pub async fn post_message(username:String,message:String,join_code:&str) ->  DynResult<()> {
    let url = format!("{BACKENDURL}/chatroom/post/message");
    let post = reqwasm::http::Request::post(&url)
        .header("username", &username)
        .header("message", &message)
        .header("join_code", &join_code); 
    post.send().await?;
    log!("message posted");
    Ok(())
}
pub async fn new_chatroom(name:String) -> DynResult<String>  {
    let url = format!("{BACKENDURL}/chatroom/new/randomid");
    if name.len() < 3 {
        return Ok(String::from("name must be longer be at least 3 digits"));
    }
    if name.len() > 20 {
        return Ok(String::from("name must be shorter than 20 characters"));
    }
    let post = reqwasm::http::Request::post(&url)
        .header("name",&name);
    let response = post.send().await?;
    let join_code = response.text().await.unwrap_or(String::from("error"));
    Ok(format!("created chatroom with join_code: {join_code}"))
}

pub async fn get_messages(join_code:&str) -> Vec<[String;2]> {
    let url = format!("{BACKENDURL}/chatroom/messages/{join_code}");
    let post = reqwasm::http::Request::get(&url);
    let response = post.send().await.unwrap();
    let json = response.json::<Messages>().await.unwrap();
    let messages = json.messages[0].messages.iter().map(|f| {
        let m = f.clone();
        let m = &m[1..m.len()-1];
        let split = m.split(",").collect::<Vec<&str>>();
        let r = [
            split[0] .to_string() .replace("φ", ","),
            split[1].to_string().replace("φ", ",")
        ];
        log!(format!("{r:?}"));
        r
    }).collect::<Vec<[String;2]>>();
    filter_old_messages(join_code).await.unwrap();
    return messages;
}
pub async fn filter_old_messages(join_code:&str) -> DynResult<()> {
    let url = format!("{BACKENDURL}/chatroom/messages/filter/{join_code}");
    reqwasm::http::Request::delete(&url).send().await?;
    Ok(())
}
pub async fn get_name(join_code:&str) -> DynResult<String> {
    let url = format!("{BACKENDURL}/chatroom/name/{join_code}");
    let post = reqwasm::http::Request::get(&url);
    let response = post.send().await.unwrap();
    let text = response.text().await?;
    Ok(text)
}

