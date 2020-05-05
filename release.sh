#!/usr/bin/bash

set output_path="$HOME/servers/actixmpp"

if [ $1 ]; then
    set output_path="$1"
fi

rm "$output_path/app"
rm -rf "$output_path/templates"
rm -rf "$output_path/static"

cargo build --release
tsc

cp target/release/actixmpp "$output_path/app"
cp -r templates "$output_path/templates"
cp -r static "$output_path/static"

sudo systemctl restart actixmpp

echo "Remember to reconfigure $output_path/Conf.json."
