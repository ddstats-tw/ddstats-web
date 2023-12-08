# ddstats-web


## Configuration
DDStats can be configured through environment variables or with a `.env` file. You can find a template in the `template.env` file which you can copy or rename to `.env`.

| Variable      | Default                      | Description                                |
| ------------- | ---------------------------- | ------------------------------------------ |
| `PORT`        | `12345`                      | The port where DDStats will run.           |
| `DB_PLAYTIME` | `../scripts/db/playtime.db`  | Location of the playtime database.         |
| `DB_DDNET`    | `../scripts/db/ddnet.sqlite` | Location of the ddnet database.            |
| `DB_POINTS`   | `../scripts/db/points.db`    | Location of the ranked points database.    |
| `LOG_PATH`    | `./ddstats.log`              | Path to write log file                     |
| `NODE_ENV`    | `development`                | Set to `production` to improve performance |