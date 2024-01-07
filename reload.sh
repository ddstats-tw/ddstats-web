git pull
pm2 restart ddstats
varnishadm 'ban req.url ~ .'
redis-cli eval "for _,k in ipairs(redis.call('keys','cache:*')) do redis.call('del',k) end" 0
