## アカウント管理アプリ
一連のユーザー管理機能を提供するアプリケーションです。

詳しくはドキュメントを参照。

### 環境
- docker
#### 構築手順
- 環境変数ファイルの作成
- ```rust-account-management/root/env/.envを作成する```

ファイル内容
```
SERVER_ADDR=0.0.0.0:3000
PG.USER=postgres
PG.PASSWORD=postgres
PG.HOST=rust-account-management-postgres
PG.PORT=5432
PG.DBNAME=rust_account_management
PG.POOL.MAX_SIZE=16
```

構築コマンド
```
docker compose up -d --build
```

### テスト実行
```
```
