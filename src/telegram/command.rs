use std::str::FromStr;

use teloxide::{
    payloads::SendMessageSetters,
    requests::{Requester, ResponseResult},
    types::{
        InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, Message, ReplyMarkup,
        WebAppInfo,
    },
    utils::command::BotCommands,
    Bot,
};
use url::Url;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum RacerCommand {
    #[command(description = "type /start to launch mini app.")]
    Help,
    #[command(description = "launch mini app.")]
    Start,
}

pub async fn answer(bot: Bot, msg: Message, cmd: RacerCommand) -> ResponseResult<()> {
    match cmd {
        RacerCommand::Help => {
            bot.send_message(msg.chat.id, RacerCommand::descriptions().to_string())
                .await?;
        }
        RacerCommand::Start => {
            let username = msg
                .from
                .map(|user| user.username.clone().unwrap_or(user.full_name()))
                .unwrap_or_default();

            let text = format!(
                r###"Hi, @{username}! This is Purr.fund Mini-app!

Click "Open app" below, then earn more Purr Points.

Invite your frens and earn a 15% point cashback through your referral system.

Earn more Points to unlock power

Let's go ⬇️"###
            );

            let mini_app_url = Url::from_str("https://mini-app-purr.vercel.app/").unwrap();

            let buttons = [InlineKeyboardButton::new(
                "Open app",
                InlineKeyboardButtonKind::WebApp(WebAppInfo { url: mini_app_url }),
            )];

            let keyboard = InlineKeyboardMarkup::default().append_row(buttons);

            bot.send_message(msg.chat.id, text)
                .reply_markup(ReplyMarkup::InlineKeyboard(keyboard))
                .await?;
        }
    };

    Ok(())
}
