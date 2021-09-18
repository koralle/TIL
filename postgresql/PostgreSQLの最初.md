# PostgreSQLやっていくぞ

## PostgreSQLってなんだ

- オープンソースのRDBMS
- 大学のプロジェクトから誕生した
- POSTGRES => Postgres95 => PostgresSQLという名前の変遷
- 現在はよく**Postgres**と呼ばれるらしい


## PostgreSQL動かしてみる

`docker-compose.yaml`を書けば一発でDockerコンテナを作れる

```yaml
version: "3.8"

services:
  postgresql:
    image: postgres:13.4
    container_name: postgresql
    ports:
      - 5432:5432
    volumes:
      - ./init:/docker-entrypoint-initdb.d
    environment:
      POSTGRES_USER: root
      POSTGRES_PASSWORD: root
      POSTGRES_INITDB_ARGS: "--encoding=UTF-8"
    hostname: postgres
    restart: always
    user: root
```

データベースを作る

`createdb koralle`

作ったデータベースにアクセスする

`psql koralle`

バージョンを確認する

```psql
koralle=# SELECT version();
                                                     version                                                      
------------------------------------------------------------------------------------------------------------------
 PostgreSQL 13.4 (Debian 13.4-1.pgdg100+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 8.3.0-6) 8.3.0, 64-bit
```

ちょっと古いけどよく使うコマンドはここにまとまっている

[[PostgreSQL] よく使うコマンドまとめ](https://dev.classmethod.jp/articles/postgresql-organize-command/)
