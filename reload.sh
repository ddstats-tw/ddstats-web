git pull
pm2 restart ddstats
varnishadm 'ban req.url ~ .'
# TODO: Add clearing of cached SQL queries in Redis.