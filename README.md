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

## structure
```
domain
=> API上でのデータ定義をし、受け渡しをする。

controller
=> サービスを実行し、ドメインをレスポンスへ渡す。

service
=> repositoryを通してドメインを取得し、整形する。

repository
=> DBの操作を行い、ドメインを返す。

response
=> ドメインからAPIレスポンスを生成する。

request
=> APIへのリクエストパラメータを管理する。

tests
=> 単体テストする。
```