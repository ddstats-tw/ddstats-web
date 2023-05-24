# ddstats/web


## Configuration
DDStats can be configured through environment variables or with a `.env` file.

| Variable      | Default                      | Description                             |
| ------------- | ---------------------------- | --------------------------------------- |
| `PORT`        | `12345`                      | The port where DDStats will run.        |
| `DB_PLAYTIME` | `../scripts/db/playtime.db`  | Location of the playtime database.      |
| `DB_DDNET`    | `../scripts/db/ddnet.sqlite` | Location of the ddnet database.         |
| `DB_POINTS`   | `../scripts/db/points.db`    | Location of the ranked points database. |