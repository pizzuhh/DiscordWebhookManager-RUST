#[allow(unused_variables)]
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
use std::process::exit;
use regex::Regex;
use reqwest::{Client, Request, Response, Url};
use tokio::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use async_recursion::async_recursion;
use serde::de::Unexpected::Str;
use colored::*;


fn main()
{
    //define string to store the URL


    match File::open("./load.json")
    {
        Ok(file) =>
            {
                println!("{}", "Loading URL from \"load.json\"".green().bold());
                let mut buf_reader = BufReader::new(file);
                let mut contents:String = String::new();
                buf_reader.read_to_string(&mut contents).expect("can't read \"load.json\"");
                let token:serde_json::Value = serde_json::from_str(&contents).expect("couldn't parse json str");
                //println!("{:?}", token);

                if let Some(t) = token.get("token")
                {
                    if let Some(tok) = t.as_str()
                    {
                        chk_url(tok);
                    }
                }

            },
        Err(_) =>
            {
                println!("{}", "File \"load\" doesn't exist or can't be opened..!".red());
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
    }




}

async fn print_info(json_d:Value)
{
    print!("----INFO----\n");
    let idd: &str;
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
            idd = webhookid;
            println!("webhook_id -> {}", webhookid);
        }
        else { idd = "0" }
    }else { idd="0" }

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
        if let Some(avv) = avatar.as_str()
        {
            let wav = format!("https://cdn.discordapp.com/avatars/{}/{}", idd, avv);
            println!("webhook_avatar -> {}", wav);
        }
        else { println!("webhook_avatar -> https://cdn.discordapp.com/avatars/{}/(null) -- webhook uses default avatar", idd) }
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
    print!("----------\n");
}

async fn getinfo_n_jumptomm(url:&str)
{

    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await.expect("error");


    if response.status().is_success()
    {
        let txt:String = response.text().await.expect("error");
        let json_d: serde_json::Value = serde_json::from_str(&txt).expect("error");
        print_info(json_d).await;
        main_menu(url).await;
    }
    else
    {
        println!("Couldn't fetch webhook info!\nMake sure the webhook exists!");
        exit(2);
    }

}

fn chk_url(url:&str)
{
    println!("Checking URL => {}", url);
    let url_chk = Regex::new(r"https://discord.com/api/webhooks/(\d{19})/(\w+)");
    match url_chk.expect("error").is_match(url)
    {
        true =>
            {
                println!("Link is valid! Proceeding to main_menu()!");
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(getinfo_n_jumptomm(url));
            }
        false => println!("Invalid link!!")
    }
}
#[async_recursion]
async fn main_menu(url:&str)
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
        2=>
            {
                let mut p:String = String::new();
                print!("Enter path> ");
                stdout().flush().expect("error");
                stdin().read_line(&mut p).expect("error");
                let mut p = p.trim();
                load_json(url, p).await;
            },   //json load

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
                    println!("{}", "Webhook deleted!".green().bold());
                    exit(0);
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
                println!("{}", "Message sent!".green().bold());
                main_menu(url).await;
            }, Err(_) => println!("bad")
    }
}

async fn load_json(url:&str, path:&str)
{
    let data:String = fs::read_to_string(path).expect("Couldn't read file!");
    let r = Regex::new(r"\n").unwrap();
    let ret = r.replace_all(&data, "");

    let client = Client::new();
    let response = client
        .post(url)
        .body(ret.to_string())
        .header("Content-Type", "application/json")
        .send()
        .await.expect("error")
        .text()
        .await;
    match response
    {
        Ok(response) =>
            {
                println!("{}", "Message sent!".green().bold());
                main_menu(url).await;
            }, Err(_) => println!("bad")
    }
}
