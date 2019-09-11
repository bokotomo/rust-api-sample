# ゲームクリエイターSNSのAPI


# 最初に
```
cp .env.sample .env

cd infrastructure/docker

docker-compose up -d

docker-compose exec rust-actix-web bash
```

# 動くか確認
```
// dockerでなくlocalで実行
curl localhost:9999/health
```
