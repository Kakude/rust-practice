# rust-practice

## Migration

### マイグレーション実行

以下コマンドでスキーマの作成および`migration`配下のマイグレーションファイルを実行する。

```shell
docker-compose exec app diesel setup
docker-compose exec app diesel migration run
```

### マイグレーションの再実行

マイグレーションファイルの誤りなどで修正を加えた場合は以下でマイグレーションを再実行する。

```shell
docker-compose exec app diesel migration redo
```

### マイグレーションファイルの作成

以下コマンドを実行すると、`migrations`ディレクトリに`<table_name>`が作成されるので、`up.sql`と`down.sql`にDDLを記載する。

```shell
docker-compose exec app diesel migration generate <table_name>
```

### DBアクセス情報の修正

`.env`ファイルを修正する。(デフォルトは以下)

```text
DATABASE_URL=mysql://root:password@db:3306/practice
```
