use rand::Rng;

pub fn lesson() {
    println!("");
    println!("##### section8だよ");

    crates();
    modules();
    multi_modules();
    binary_crates();
    documents();
    build_profiles();
}

fn crates() {
    /*
    ■クレートとは
    ・Rustのプログラムを構成する要素
    ・ライブラリもしくは実行ファイルのソースコード、関連するテスト、ツール、設定など全てが収められる
    ・バイナリクレート：実行が可能なクレート
    ・ライブラリクレート：バイナリクレートから機能として呼び出されることで実行されるクレート

    ■パッケージ
    ・ある機能群を提供する1つ以上のクレートの集合　クレートより大きい単位
    ・cargo new プロジェクト名で作成したもの。Cargo.tomlにパッケージ情報を記述
    ・パッケージに含むことができるクレート数は
        - ライブラリクレート：0か1個
        - バイナリクレート：0個以上

    ・Cargoはsrc/main.rs をバイナリクレートと認識し、src/lib.rs をライブらいrクレートと認識する。
        - cargo new では1つのバイナリクレートが作成される
    ・src/bin ディレクトリにファイルを置くことで、それぞれのファイルが別のバイナリクレートになる。

    ■クレートの依存関係
    ・あるクレートから別のクレートを呼び出して使用できる
        - crate.ioから他のクレートを取得したり公開できる
    ・crate.ioからクレートを取得するには、Cargo.tomlの[dependencies]セクションに使用したいクレート名とバージョンを指定する
        - するとcargo run, cargo bulld時にcrate.ioからダウンロードされる。

    例： rand 0.8,5 を検索。コピーボタンがあるのでCarrgo.tomlに貼り付ける。



    */
    // 最初のビルド時にダウンロードでしばらく時間が掛かる。：その後randが使えるようになる。
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("{}", random_number);
}

fn modules() {
    /*
    ■モジュールとは
    ・クレート内のコードをグループ化、可読性と再利用性を高める機構
        - 関数、構造体、列挙型、定数などが収められる
    ・Rustの名前空間として使用される
    ・mod キーワードを使用して作成
    ・1ファイル=1モジュールが基本、モジュールが肥大化してきたら複数ファイルに分割。

    ■モジュールの可視性
    ・モジュール内容その可視性を制御可能
        - public: 要素がモジュール外部からアクセス可能　pubキーワードを付ける
        - private: モジュール内もしくは子モジュールからアクセス可能。デフォルトはこちら。

    */

    pub mod test_module {
        pub fn test_fn1() {}
        // fn test_fn2() {
        // }
        pub mod sub_module1 {
            pub fn test_sub_fn1() {}
        }
    }

    test_module::test_fn1();
    //test_module::test_fn2();// function `test_fn2` is private
    test_module::sub_module1::test_sub_fn1();
    //crate::section8::sec8main::test_module::sub_module1::test_sub_fn1(); だめだった

    /*
    ■モジュールのネスト
    ・内側のモジュールをサブモジュールと呼ぶ
    ・ネストされたモジュール要素を外部から使うには、その要素だけでなくサブモジュールも全部publicに
    ・pub(super)とすると、親モジュールからのみアクセス可能になる。

    ■パス
    ・パスの指定が必要。2種類。区切りは ::
        - 絶対パス：クレートの名前か crate という文字列
        - 相対パス：self, super や今のモジュール内の識別子を使って現在のモジュールから始まる

    ■インポート
    ・モジュールの要素をインポート、局所的なエイリアスが作成される。
    ・ main関数の上で use キーワード
    ・直接インポートでなく、要素を含むモジュールやトレイとなどを使うのが良いと言われる。

    */
}

fn multi_modules() {
    /*
    ■モジュールの複数ファイルへの分割
    main.rsでやっている

    構造体も pub をつけられる。しかしフィールドはprivateのまま。フィールドひとつひとつにpubをつけるか、コンストラクタメソッドをpubにする。
    OOPのカプセル化と同じ。
    */
}

fn binary_crates() {
    /*
    rust_lesson パッケージの src/main.rs がバイナリクレートと認識されている。
    bin/ にmain関数を持ったファイルがあると複数になる。

    > cargo run --bin rust_lessson
    > cargo run --bin bin1
    > cargo run --bin bin2

    */
}

fn documents() {
    /*
    ドキュメント
    > cargo doc --no-deps --open

    --no-depsをしないと、Cargo.tomlにあるdependenciesのクレートの分も生成してしまう。
    pubをつけた関数のみ。ドキュメントコメントで書いたもの。

    rust_lesson/target/doc/rust_lesson/index.html
    rust_lesson/target/doc/bin1/index.html
    rust_lesson/target/doc/bin2/index.html

    */
}

fn build_profiles() {
    /*
    > cargo run --bin rust_lesson --release  //本番用。

    普段は1．36sだったのが 4．94s掛かっている。4倍ぐらいか。
    target/debug/ のほかに /release/ もできる。標準出力するプログラムだと一瞬なので実行速度同じ。


     */
}
