git pull
cargo build --release
systemctl --user restart ddstats
varnishadm 'ban req.url ~ .'
