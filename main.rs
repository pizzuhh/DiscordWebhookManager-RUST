#[allow(unused_variables)]
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
use std::process::exit;
use regex::Regex;
use reqwest::{Client, Request, Response};
use tokio::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;


fn main()
{
    //define string to store the URL
    let mut url:String = String::new();
    //Get user input
    print!("Enter Webhook URL> ");
    stdout().flush().expect("Failed to flush\n");
    match stdin().read_line(&mut url)
    {
        Ok(_) =>
            {
                let url:&str = url.trim();
                chk_url(url);
            }, Err(_) => println!("fail")
    }
}

fn print_info(json_d:Value)
{

    //guild_id
    if let Some(guild_id) = json_d.get("guild_id")
    {
        if let Some(gid) = guild_id.as_str()
        {
            println!("guild_id -> {}", gid);
        }
    }

    //channel_id
    if let Some(channel_id) = json_d.get("channel_id")
    {
        if let Some(channelid) = channel_id.as_str()
        {
            println!("channel_id -> {}", channelid);
        }
    }

    //webhook_id
    if let Some(webhook_id) = json_d.get("id")
    {
        if let Some(webhookid) = webhook_id.as_str()
        {
            println!("webhook_id -> {}", webhookid);
        }
    }

    //webhook_name
    if let Some(webhook_name) = json_d.get("name")
    {
        if let Some(webhookname) = webhook_name.as_str()
        {
            println!("name -> {}", webhookname);
        }
    }
    //webhook_avatar
    if let Some(avatar) = json_d.get("avatar")
    {
        if let Some(av) = avatar.as_str()
        {

        }
    }

    //user array
    if let Some(usr_arr) = json_d.get("user")
    {
        //username
        if let Some(username) = usr_arr.get("username")
        {
            if let Some(usrn) = username.as_str()
            {
                println!("username -> {}", usrn);
            }
        }

        //id
        if let Some(id) = usr_arr.get("id")
        {
            if let Some(usrid) = id.as_str()
            {
                println!("user_id -> {}", usrid);
            }
        }

    }
}

async fn getinfo_n_jumptomm(url:&str)
{
    //println!("{url}"); //check if everything is fine

    let response: String = reqwest::Client::new()
        .get(url)
        .send()
        .await.expect("error")
        .text()
        .await.expect("error");
    //let json_res:serde_json::Value = response.json().unwrap();
    let json_d: serde_json::Value = serde_json::from_str(&response).expect("error");
    print_info(json_d);
    MainMenu(url).await;
}

fn chk_url(url:&str)
{
    println!("Checking URL => {}", url);
    let url_chk = Regex::new(r"https://discord.com/api/webhooks/(\d{19})/(\w+)");
    match url_chk.expect("error").is_match(url)
    {
        true =>
            {
                println!("Link is valid! Proceeding to MainMenu()!");
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(getinfo_n_jumptomm(url));
            }
        false => println!("Invalid link!!")
    }
}

async fn MainMenu(url:&str)
{
    println!("1. Send Message\t\t\t2. Send message by loading json file\n3. Delete webhook\t\t4. Exit");
    let mut c:String = String::new();
    print!("> ");
    stdout().flush().expect("Failed to flush\n");

    stdin().read_line(&mut c).expect("error");
    let p:u8 = c.trim().parse::<u8>().expect("err");

    match p
    {
        1=>
        {
            let mut input:String = String::new();
            print!("message> ");
            stdout().flush().expect("err");
            stdin().read_line(&mut input).expect("error");
            let input = input.trim();
            let json_d:String = format!("{{\"content\":\"{}\"}}", input);
            send_message(url, json_d).await;
        },   //sendmsg
        2=>println!("1"),   //json load
        3=>
            {
                delete_webhook(url).await;
            },   //delete
        4=>exit(0),
        _=>println!("invalid option"),
    }
}

async fn delete_webhook(url:&str)
{
    let client = Client::new();
    let response= client.delete(url).send().await;
    match response
    {
        Ok(response)=>
            {
                if response.status().is_success()
                {
                    println!("Webhook deleted!");
                }
                else { println!("bad11") }
            }, Err(_) => println!("bad")
    }

}

async fn send_message(url:&str, json_d:String)
{
    let client = Client::new();
    let response = client
        .post(url)
        .body(json_d)
        .header("Content-Type", "application/json")
        .send()
        .await.expect("error")
        .text()
        .await;
    match response
    {
        Ok(response) =>
            {
                println!("Message sent!");
            }, Err(_) => println!("bad")
    }
}