![Bulbbot Banner](./assets/BannerWide.png)

# Bulbbot-rs

> Still work in progress

A powerful Discord moderation bot with rich features allowing server Moderators and Administrators to manage their communities. Now written in Rust.

---

## Getting started with Bulbbot

Click [here](https://bulbbot.rocks//invite) to invite Bulbbot to your server.

The bot only uses [slash commands](https://docs.bulbbot.rocks/basics/slash-commands) `/` which is the future of Discord bot commands.

For a full list of commands [click me](https://docs.bulbbot.rocks/command-list)

## Support

Whether it is a command that's not working, your server configuration being not quite right or maybe you've found a bug in our code out in the wild we're ready to assist you with all your technical
issues and questions over at our [support server](https://bulbbot.rocks/discord)

## Meet the Team

[Meet the users who have built Bulbbot](https://docs.bulbbot.rocks/team)

### Packages

**!!! We offer no support in selfhosting the bot !!!**

While we offer the code to be public for respect to our users, we are not able to provide you with any help with selfhosting the bot, but we will provide our own documentation for our own developers to setup their environment that you can use to figure out it on your own. https://docs.bulbbot.rocks/guides/setup-dev

#### [bulbbot.Babelz](bulbbot.Babelz/)

Our translation library (a bit work in progress). To get the translations we utilize [Crowdin](https://crowdin.com/) that exports the `.yml` files to us.

#### [bulbbot.Commands](bulbbot.Commands/)

TBA but will be handling the interaction requests sent from Discord.

#### [bulbbot.Gateway](bulbbot.Gateway/)

Built with Rust to handle the Discord gateway events sent to the bot in a scalable way.

#### [examples](examples/)

Code that can be used as refrence during development.

---

**License**
Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License
