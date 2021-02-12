pub mod cmd_error;
pub mod cmd_help;
pub mod cmd_info;
pub mod cmd_prefix;
pub mod cmd_search;
pub mod globals;

use crate::{
    cmd_info::CMD_INFO_COMMAND,
    cmd_prefix::CMD_PREFIX_COMMAND,
    cmd_search::{
        CMD_DICTIONARY_COMMAND, CMD_GOOGLE_COMMAND, CMD_IMAGE_COMMAND, CMD_URBAN_COMMAND,
    },
};
use globals::{BotConfig, BotInfo};
use serenity::{
    builder::CreateEmbed,
    client::{Context, EventHandler},
    framework::standard::macros::group,
    model::{channel::Message, id::GuildId, misc::Mentionable, prelude::Activity},
};
use std::{fmt::Display, io::Write};

#[group("Master")]
#[sub_groups(General, Search)]
#[help_available(false)]
struct Master;

#[group("General Stuff")]
#[commands(cmd_info, cmd_prefix)]
struct General;

#[group("Search Things")]
#[commands(cmd_google, cmd_image, cmd_dictionary, cmd_urban)]
struct Search;

pub struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _: Vec<GuildId>) {
        ctx.set_activity(Activity::playing(
            format!(
                "@{} help",
                BotInfo::get().expect("Couldn't get BotInfo").name()
            )
            .as_str(),
        ))
        .await;

        println!("Connected!");
        log(&ctx, &String::from("Connected!")).await;
    }
}

pub async fn send_embed(ctx: &Context, reply: &Message, is_error: bool, mut embed: CreateEmbed) {
    let channel = reply.channel_id;
    if is_error {
        embed.colour(11534368);
    } else {
        match BotConfig::get() {
            Some(config) => {
                embed.colour(config.colour());
            }
            None => log(ctx, "Couldn't get BotConfig to get colour").await,
        };
    };

    if let Err(err) = channel.send_message(ctx, |m| m.set_embed(embed)).await {
        if let Err(err) = channel
            .say(ctx, format!("Oops, couldn't send the message 🤦‍♀️: {}", err))
            .await
        {
            if let Err(err) = reply
                .author
                .dm(ctx, |m| {
                    m.embed(|e| {
                        e.colour(11534368)
                            .description(format!(
                                "{}\nLet the admins know so they can fix it\n",
                                err
                            ))
                            .title(format!(
                                "Looks like I can't send messages in {} :(",
                                reply.channel_id.mention()
                            ))
                    })
                })
                .await
            {
                log(
                    ctx,
                    format!(
                        "Couldn't even send the message to inform the commander: {}",
                        err
                    ),
                )
                .await
            }
        }
    }
}

pub async fn log(ctx: &Context, msg: impl Display + AsRef<[u8]>) {
    match BotInfo::get() {
        Some(info) => match info.owner().create_dm_channel(ctx).await {
            Ok(channel) => {
                if let Err(err) = channel.say(ctx, &msg).await {
                    print_and_write(format!(
                        "Couldn't DM the owner when trying to log: {}\nMessage: {}",
                        err, msg
                    ));
                }
            }
            Err(err) => print_and_write(format!(
                "Couldn't get the DM channel with the owner when trying to log: {}\nMessage: {}",
                err, msg
            )),
        },
        None => print_and_write(format!(
            "Couldn't get BotInfo when trying to log\nMessage: {}",
            msg
        )),
    };
}

pub fn print_and_write(msg: impl Display) {
    let mut print_and_write = format!(
        "{}: {}\n\n",
        chrono::Utc::now().format("%e %B %A %H:%M:%S"),
        msg
    );
    println!("{}", print_and_write);

    let log_file = match BotConfig::get() {
        Some(config) => config.log_file(),
        None => {
            print_and_write += "Writing into a file named \"discord-base logs.txt\" because getting BOT_CONFIG also failed\n\n";
            "discord-base logs.txt"
        }
    };

    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
    {
        Ok(mut file) => {
            if let Err(err) = file.write(print_and_write.as_bytes()) {
                println!("Couldn't write to the log file: {}", err)
            }
        }
        Err(err) => println!("Couldn't open or create the log file: {}", err),
    }
}
