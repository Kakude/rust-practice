# rust-practice

## Migration

### diesel_cliのインストール

※ MYSQLクライアントがインストールされている前提(`brew install mysql`などでインストール)

```shell
cargo install diesel_cli --no-default-features --features mysql
```

### マイグレーション実行

以下コマンドでスキーマの作成および`migration`配下のマイグレーションファイルを実行する。

```shell
diesel setup & diesel migration run  
```

### マイグレーションの再実行

マイグレーションファイルの誤りなどで修正を加えた場合は以下でマイグレーションを再実行する。

```shell
diesel migration redo
```

### マイグレーションファイルの作成

以下コマンドを実行すると、`migrations`ディレクトリに`<table_name>`が作成されるので、`up.sql`と`down.sql`にDDLを記載する。

```shell
diesel migration generate <table_name>
```

### DBアクセス情報の修正

`.env`ファイルを修正する。(デフォルトは以下)

```text
DATABASE_URL=mysql://root@localhost:3331/practice
```