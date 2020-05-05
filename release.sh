#!/usr/bin/bash

rm "$HOME/servers/actixmpp/app"
rm -rf "$HOME/servers/actixmpp/templates"
rm -rf "$HOME/servers/actixmpp/static"

cargo build --release
tsc

cp target/release/actixmpp "$HOME/servers/actixmpp/app"
cp -r templates "$HOME/servers/actixmpp/templates"
cp -r static "$HOME/servers/actixmpp/static"

sudo systemctl restart actixmpp
