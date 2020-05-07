#!/usr/bin/bash

export output_path="$HOME/servers/actixmpp"

! [ -z ${1+x} ] && (
    export output_path="$1"
    echo fin
)

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

sleep 1.5s
systemctl status actixmpp
