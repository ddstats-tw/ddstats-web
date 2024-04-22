# ddstats-web


## Configuration
DDStats can be configured through environment variables or with a `.env` file. You can find a template in the `.env.template` file which you can copy or rename to `.env`.

## Development

### Generate styles

```
sass --style=compressed --watch --update --no-source-map scss/main.scss:static/css/main.css
```
