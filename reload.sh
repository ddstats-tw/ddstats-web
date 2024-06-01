git pull
sass --style=compressed --update --no-source-map scss/main.scss:static/css/main.css
cargo build --release
systemctl --user restart ddstats
varnishadm 'ban req.url ~ .'
