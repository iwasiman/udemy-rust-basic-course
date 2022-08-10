pub fn lesson() {
    println!("");
    println!("##### section10だよ エラー処理");

    /*
    panic 起こっては行けないエラーのとき。配列の未定義indexへのアクセス、0で割るなど
    すべてが巻き戻せられる

    thread 'main' panicked at 'panicおこすよ', src/section10/sec10main.rs:10:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    //panic!("panicおこすよ");

    println!("{:?}", need_even(2)); //Ok(2)
    println!("{:?}", need_even(1)); // Err("引数は偶数にしてちょ")

    let x = match need_even(2) {
        Ok(val) => val,
        Err(err) => {
            println!("Err型が返ってきたよ メッセージ： {}", err);
            panic!()
        }
    };
    println!(" x= {:?}", x); // 2
    // let y = match need_even(111) {
    //     Ok(val) => val,
    //     Err(err) => {
    //         println!("Err型が返ってきたよ メッセージ： {}", err);
    //         panic!()
    //     }
    // };
    // println!(" y= {:?}", y); // panicが起こるのでここまで到達しない。

    // 上のような書き方は冗長なので用意されているものがある。
    // let s10 = need_even(10);
    // let s9 = need_even(9);
    println!("Result型s10のis_ok {}", need_even(10).is_ok()); // true
    println!("Result型s10のis_err {}",need_even(10).is_err()); // false
    println!("Result型s10のok {:?}", need_even(10).ok()); // Some(10)
    println!("Result型s10のerr {:?}", need_even(10).err()); // None 変数s10に入れてok()を呼んだあとだと所有権移動後でエラーになる
    println!("Result型s9のok {:?}", need_even(3).ok()); // None
    println!("Result型s9のerr {:?}", need_even(3).err()); // Some("引数は偶数にしてちょ")

    println!("Result型s10にunwrap_or {:?}", need_even(10).unwrap_or(0)); // Ok(10)が関数から戻るので10
    println!("Result型s9にunwrap_or {:?}", need_even(3).unwrap_or(-1)); // 引数の-1が戻る
    // unwrap() だと、成功時はOkバリアントの中身を返し、失敗時はPanic expectだとメッセージを渡せる
    //println!("Result型s9にunwrap {:?}", need_even(3).unwrap()); // パニック発生
    //println!("Result型s9にunwrap {:?}", need_even(3).expect("expectから発生！")); // thread 'main' panicked at 'expectから発生！: "引数は偶数にしてちょ"', src/section10/sec10main.rs:48:54

/*
エラーの移譲
 */
    println!("double_even関数の戻り値 {:?}", double_even(1)); // Err("引数は偶数にしてちょ")
    println!("double_even2関数の戻り値 {:?}", double_even2(4)); // Ok(8)
    match double_even2(1) {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("呼び出し元でハンドリング"); // 両方のメッセージが出力される。
            println!("{}", err)
        }
    }


}

// Result型を使った関数
fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてちょ"))
    }
}

fn double_even(b: i32) -> Result<i32, String> {
    match need_even(b) {
        Ok(val) => Ok(val * 2),
        Err(err) => Err(err),
    }
}

fn double_even2(b: i32) -> Result<i32, String> {
    let x = need_even(b)?; // 右辺でOkバリアントが返ってきたらxに代入。Errだったらその場で終了。Result型を返す関数の中でしか使えない。
    Ok(x * 2)
}