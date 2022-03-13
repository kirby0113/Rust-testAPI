## 概要
- 開発用のメモです。

## 参考資料

### Rust

- [RustのOptionとResult](https://qiita.com/take4s5i/items/c890fa66db3f71f41ce7)
```
fn get_value_good(v: bool) -> Option<usize> {
    if v {
        Some(100)
    } else {
        None
    }
}

match get_value_good(true) {
        Some(result) => println!("success: {}", result),
        None => println!("failure"),
    }
```

- [Rustの構造体などに追加できる振る舞いを確認する](https://qiita.com/apollo_program/items/2495dda519ae160971ed)
```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoComments {
    pub todo_id: i32,
    pub comments: Vec<TodoComment>
}
```

### Actix-web
- [依存関係 v4.0.1](https://crates.io/crates/actix-web/4.0.1/dependencies)
- [Actix Web](https://actix.rs/)
- [Rust で actix-web を使って REST API を実装していく](https://qiita.com/Yoshihiro-Hirose/items/2426fe5199cb1ff74bd7)
- [actix-webでmongodbを利用する](https://qiita.com/deepgreenAN/items/28e8d41a71620842060d)

#### example

##### サーバーの起動
```
    HttpServer::new(|| App::new().configure(routes::routes)) //routesは他ファイルで定義したルーティング
        .bind("IP Address and Port")?
        .run()
        .await
```

### MongoDB
- [開発用データベース](https://cloud.mongodb.com/v2/622db946a2407f364c4c2e4d#clusters)
- [公式ドキュメント](https://docs.mongodb.com/)
- [MongoDB-Rust-Driver Github](https://github.com/mongodb/mongo-rust-driver/)

#### DB取得等

##### クライアントの取得
```
let client_options = ClientOptions::parse(
        "mongodb+srv://UserName:Password@rust-testapi.kxdjy.mongodb.net/Rust_testAPI?retryWrites=true&w=majority",
    )
    .await.expect("Database Link Error.");

// Get a handle to the deployment.
let client = Client::with_options(client_options).expect("get Client Error.");
```

##### データベース・コレクションの取得
```
let database = client.database(DatabaseName);
let collection = database.collection::<Document>(CollectionName);

```

#### CRUD
##### データの登録
```
let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
];

collection.insert_many(docs, None).await.expect("insert error.");
```


