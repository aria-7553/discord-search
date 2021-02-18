# discord-search
[![](https://img.shields.io/static/v1?color=f48fb1&labelColor=f48fb1&label=discord&message=add%20to%20your%20server&logo=discord&logoColor=ffffff&style=for-the-badge)](https://discord.com/api/oauth2/authorize?client_id=752582273706098699&permissions=117824&scope=bot)  
[![](https://img.shields.io/static/v1?color=f48fb1&labelColor=f48fb1&label=discord‎‎‎‎‎‎‎‎‎‎‎‎‎‎‎‎‎‎&message=join%20my%20server&logo=discord&logoColor=ffffff&style=for-the-badge)](https://discord.gg/u6NyRUnNED)  

A minimalist Discord bot to search Google, Google Images, Wiktionary and Urban Dictionary  
Made with love using [Serenity](https://github.com/serenity-rs/serenity) in Rust!

## Download it

*(Your OS may tell you that it's insecure. That's the case for most non-popular open source projects. If you're still sceptical you can build it from the source)*

### Windows 64-bit
https://github.com/aria-7553/discord-search/releases/download/1.1.2/discord-search-1.1.2-windows.exe
### macOS 64-bit
https://github.com/aria-7553/discord-search/releases/download/1.1.2/discord-search-1.1.2-macos
### Linux 64-bit
https://github.com/aria-7553/discord-search/releases/download/1.1.2/discord-search-1.1.2-linux

## Use it
1. Click on the file you downloaded from [the Download it section](#download-it) or `cd` into its directory and type its name *(On macOS and Linux you might have to [make the file executable](https://support.apple.com/en-nz/guide/terminal/apdd100908f-06b3-4e63-8a87-32e71241bab4/2.11/mac/11.0))*
2. It'll create the config file ending with `config.toml` where the file is *(If it isn't there, read the stuff in the window to see the errors)*
3. Edit it with your text editor *(Notepad, TextEdit, nano etc.)* *(Instructions are inside)*
3. Go to [the application page](https://discord.com/developers/applications), select your bot and set the description. The `info` command will use that and the account that has that application
4. Click on the file again to run it. It'll open in a terminal, close it and the bot shuts down

### How to build it (If you can't find your platform at [the Download it section](#download-it))
On Windows, this requires about 6GB of download, *because Microsoft..*  
On other platforms though, it takes about 5 minutes
1. Follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust
- On Windows, follow the instructions in the window that opens
- On Linux, install required tools from [here](https://ostechnix.com/install-development-tools-linux/)
2. Clone this repository (Download all the files into a folder)
3. Open your terminal and `cd` into to the folder with `Cargo.toml` in it
4. Type `cargo build --release` in the terminal
5. Wait for a message starting with `Finished`
6. You'll find the executable in `target/release/`
7. *(On macOS and Linux you might have to [make the file executable](https://support.apple.com/en-nz/guide/terminal/apdd100908f-06b3-4e63-8a87-32e71241bab4/2.11/mac/11.0))*

## What it does

### Commands
All these have the prefix `.` since they aren't commands that are used frequently

These give a direct link that opens the results on that page when clicked on. This way, it doesn't flood the conversation, is much more flexible and still is just a touch/click away

#### google
- Aliases: `s, search`

#### image
Searches on Google images
- Aliases: `i, images, imagesearch, image-search, image_search`

#### dictionary
Opens the page on Wiktionary
- Aliases: `d, wiktionary, definition, define, meaning`

#### urban
Opens the page on Urban Dictionary
- Aliases: `u`

### General Commands
All these don't have a prefix so they're run with `@bot [command]`. You set your own prefix for the groups you create

*(I made it this way because usually only these commands collide with other bots so that I can use `.` as the prefix for my own commands)*

#### Help command

*(Provided by Serenity's standard framework)*

- A nice help command, listing all the other commands and their groups
- Gives more information about a command with `help [command]`
- Suggests similar commands if `help [command]` is.. similar to another command

#### Info command
- An `info` command that gets the description and owner from [the application page](https://discord.com/developers/applications) and the GitHub page and invite link from the config file

#### Prefix command
- A `prefix` command that sets the prefix for the guild, which works for every command in addition to `@bot` and the prefixes you set for your groups
- This isn't as simple as it seems. It means the bot has to check if the message starts with its prefix in that server for every message that's sent
- To further optimise this, the bot first checks the message's first `max prefix length (10) + longest command's length` characters if it includes any of the commands, if not it doesn't unnecessarily check since there's no way the message includes a command

### Presence
- Sets the presence to `Playing a game: @[bot's username] help` (This looks much better than other presences Discord allows)

### Optimisation
- I've tried my best to use statics and avoid `await`s
- Also adding buckets and rate limit handling to ensure it isn't abused
- Combined with Rust's and SQLite's performance, the bot should be really lightweight

*I can't say fast because we'll be bottlenecked by Discord anyway. It's still as light fast and fast as it can be*

### Error handling
Everything that's done follows these principals:
- If the action isn't expected by the user, don't inform them even if it fails
- If anything else, do inform them. Fall back to DMing the user if informing the user failed
- If it's a user error, tell them the error and how to fix it if it's not obvious
- If it's a bot error, tell them how to report it and what they can do and inform the owner of the bot. Fall back to printing and logging in a file
- If we can't be sure, tell them the error and how to report it if it looks like a bot error  

These, combined with Rust's safety, ensure the best user experience. Most of these errors could be handled by falling back (Not using embeds, DMing the user etc.) but this overcomplicates the bot and makes it inconsistent, instead of forcing the user to fix it, so it's not a good practice

## Who I am
Just some (currently) 17 years old girl from Turkey coding  
Started with Python, gave a shot to JS but now that I know Rust exists never going back  
Basically all I did was Discord bots (at least at the time of writing)  
License and stuff I don't care, neither should you but contact me if you want to ask anything

## How you can contact me
- Very Fast: Discord: aria#7553
- Slow: GitHub issues or something
- Way slower (or never, if you're unlucky): wentaas@gmail.com


## Ideas I had but decided not to implement

### Handling permissions
Too expensive, limited, bad for UX, unnecessary and inconsistent. Doing proper error checking and informing the user on an error is just a better option

### Localisation
Makes everything more expensive, you now have to check the language for every message you're sending, which means you can't use any static string. Community translation will always be inconsistent, slow and incomplete. Most users wouldn't expect or use it. Having separate bots for each language is just a better option

### Customisation
Again makes everything more expensive, since it means you can't use any static string. If someone is hosting the bot, they most likely have enough knowledge to search for a string in the source and replace it then build. It isn't necessary at all and I still tried to include customisation when it didn't mean a performance loss
