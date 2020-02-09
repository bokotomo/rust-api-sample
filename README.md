# rust-api-sample

# 動かす
```
cp .env.sample .env

cd infrastructure/docker

docker-compose up -d

docker-compose exec rust-actix-web bash

cargo run
```

# 動いてるか確認
```
// dockerでなくlocalで実行
curl localhost:9999/health
```

# migration
```
diesel migration generate create_design

diesel migration run

diesel migration redo
```

## structure
```
domain
=> API上でのデータ定義をし、そのデータを完全に扱う。

controller
=> サービスを実行し、受け取ったドメインをレスポンスへ渡す。

service
=> repositoryを通してドメインを取得し、整形する。今回は使ってない。

repository
=> DBの操作を行い、ドメインを返す。

response
=> 「controllerから受け取ったドメイン」からAPIレスポンスを生成する。

request
=> APIへのリクエストパラメータを管理する。

driver
=> API外部との接続をする。

model
=> ORMのためのテーブル定義を管理。

usecase
=> 依存性の逆転をする。今回は使ってない。

tests
=> 単体テストする。

infrastructure
=> 環境依存のものを管理。

e2e
=> end to endテストを行う。型あり言語でなくpythonで記述。
```

## やってないこと
・usecase層で、依存性逆転(DIP)はやっていない。   
・service層も、今回は必要なかったのでやってない。     
・DIもやっていない。    