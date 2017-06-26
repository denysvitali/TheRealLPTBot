# TheRealLPTBot
The Real LPT is always in the comments.

From [an idea](https://www.reddit.com/r/LifeProTips/comments/6ixc7g/lpt_when_sharing_pictures_of_concertsporting/djai7ig/) of [/u/ktkps](https://www.reddit.com/user/ktkps) and [/u/Hi-lo](https://www.reddit.com/user/Hi-lo): we're glad to present you TheRealLPTBot, a bot that reposts real LPT found in comments to [/r/TheRealLPT](https://www.reddit.com/r/TheRealLPT/)

# Configuration
Create a file `credentials.yml` and add the following:
```
username: "BotUsername"
password: "BotPassword"
app_id: "AppID"
secret: "AppSecret"
```

## Getting an `app_id` and `secret`
To get an `app_id` and a `secret`, follow the instructions [here](https://github.com/reddit/reddit/wiki/OAuth2-Quick-Start-Example).  

TL;DR: go to [App Preferences](https://www.reddit.com/prefs/apps), create an app, add your bot as a developer and use the `app_id` + `secret` in the `credentials.yml` file)

# Run it
You'll need [Rust](https://www.rust-lang.org/) installed on your system: then you can `cargo run` it and let it run

# Special Thanks
- [/u/ktkps](https://www.reddit.com/user/ktkps) for the idea and for gilding my comment ( <3 )
