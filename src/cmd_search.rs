use crate::{log, send_embed};
use once_cell::sync::OnceCell;
use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use url::Url;

static GOOGLE: OnceCell<(CreateEmbedAuthor, Url, &'static str)> = OnceCell::new();
static IMAGE: OnceCell<(CreateEmbedAuthor, Url, &'static str)> = OnceCell::new();
static DICTIONARY: OnceCell<(CreateEmbedAuthor, Url, &'static str)> = OnceCell::new();
static URBAN: OnceCell<(CreateEmbedAuthor, Url, &'static str)> = OnceCell::new();

pub fn set_sites() {
    let mut author_google = CreateEmbedAuthor::default();
    author_google
        .name("Google")
        .url("https://www.google.com/")
        .icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/5/53/Google_%22G%22_Logo.svg/500px-Google_%22G%22_Logo.svg.png");
    let mut author_image = CreateEmbedAuthor::default();
    author_image
        .name("Google Images")
        .url("https://images.google.com/")
        .icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/5/53/Google_%22G%22_Logo.svg/500px-Google_%22G%22_Logo.svg.png");
    let mut author_dictionary = CreateEmbedAuthor::default();
    author_dictionary
        .name("Wiktionary")
        .url("https://en.wiktionary.org/")
        .icon_url("https://upload.wikimedia.org/wikipedia/commons/0/07/Wiktsister_en.png");
    let mut author_urban = CreateEmbedAuthor::default();
    author_urban
        .name("Urban Dictionary")
        .url("https://www.urbandictionary.com/")
        .icon_url("https://static.wikia.nocookie.net/logopedia/images/0/0b/UDFavicon.png");

    GOOGLE
        .set((
            author_google,
            Url::parse("https://www.google.com/search").unwrap(),
            "q=",
        ))
        .unwrap();
    IMAGE
        .set((
            author_image,
            Url::parse("https://www.google.com/search").unwrap(),
            "tbm=isch&q=",
        ))
        .unwrap();
    DICTIONARY
        .set((
            author_dictionary,
            Url::parse("https://en.wiktionary.org").unwrap(),
            "wiki/",
        ))
        .unwrap();
    URBAN
        .set((
            author_urban,
            Url::parse("https://www.urbandictionary.com/define.php").unwrap(),
            "term=",
        ))
        .unwrap();
}

async fn get_search_embed(
    ctx: &Context,
    args: Args,
    site: Option<&(CreateEmbedAuthor, Url, &'static str)>,
    is_dictionary: bool,
) -> (CreateEmbed, bool) {
    let mut embed = CreateEmbed::default();
    let term = args.rest().trim();

    if term == "" {
        embed.title("I need something to search for though..");
        return (embed, true);
    }

    let target = match site {
        Some((author, url, query)) => {
            let mut url = url.clone();
            let rest = &format!("{}{}", query, term);
            if is_dictionary {
                url.set_path(rest)
            } else {
                url.set_query(Some(rest));
            }
            Some((author, url))
        }
        None => None,
    };

    match target {
        Some((author, url)) => {
            embed
                .description(url.as_str())
                .set_author(author.clone())
                .footer(|f| {
                    f.text(
                        "Sorry it's ugly like that.. Otherwise Discord will annoy you with pop-ups",
                    )
                });
            (embed, false)
        }
        None => {
            log(ctx, "Couldn't get the search site").await;
            embed.title("Ugh, I can't find where I kept all these search links");
            (embed, true)
        }
    }
}

#[command("google")]
#[aliases("s", "search")]
#[bucket = "general"]
#[description = "Let me help you google something"]
#[usage = "[what you want me to google]"]
#[example = "what's it like to feel emotions"]
async fn cmd_google(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (embed, is_error) = get_search_embed(ctx, args, GOOGLE.get(), false).await;
    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}

#[command("image")]
#[aliases("i", "images", "imagesearch", "image-search", "image_search")]
#[bucket = "general"]
#[description = "Let me search Google images for you"]
#[usage = "[what you want me to search google images for]"]
#[example = "cute koalas"]
async fn cmd_image(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (embed, is_error) = get_search_embed(ctx, args, IMAGE.get(), false).await;
    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}

#[command("dictionary")]
#[aliases("d", "wiktionary", "definition", "define", "meaning")]
#[bucket = "general"]
#[description = "Don't know a word? Let me help you look it up on Wiktionary for you\n(Wiktionary is like the Wikipedia of words and supports practically any language! It also has all sorts of info like pronunciation, etymology, examples etc. Seriously it's great)"]
#[usage = "[what you don't know the definition of]"]
#[example = "wie geht es dir"]
async fn cmd_dictionary(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (embed, is_error) = get_search_embed(ctx, args, DICTIONARY.get(), true).await;
    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}

#[command("urban")]
#[aliases("u")]
#[bucket = "general"]
#[description = "Don't know the latest internet words and feeling like a boomer? Now I can help"]
#[usage = "[the edgy phrase you want to learn]"]
#[example = "third wheel"]
async fn cmd_urban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (embed, is_error) = get_search_embed(ctx, args, URBAN.get(), false).await;
    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}
