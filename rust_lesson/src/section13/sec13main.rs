pub fn lesson() {
    println!("");
    println!("##### section13だよ テスト");

    //test_sample(); // not found in this scope になる。

    maybe_panic(false); // テスト対象メソッド。
    add(10, 20);

}

#[test]
fn test_sample() {
    let a = 1 + 1;
    let b = 2;
    assert_eq!(a, b);
    //b = 999;
    //assert_eq!(a, b); // これはFAILEDする
    // 1テスト関数＝1アサーションが分かりやすい。

    /*
> cargo test
    Running unittests src/main.rs (target/debug/deps/rust_lesson-15f2cae33812ee51)

running 1 test
test section13::sec13main::test_sample ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rust_lesson

running 1 test
test src/lib.rs - say_goodbye (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s

---
> cargo test -- --test-threads=1  // スレッド数1、全てのテストが直列に実行される


*/
}

fn maybe_panic(flag: bool) {
    if flag == false {
        println!("safe!");
    } else {
        panic!("フラグがtrueだぞい");
    }
}

#[cfg(test)] // これをつけるとcargo build ではビルドされない。コンパイル時間削減、サイズ削減
mod test_module {
    #[test]
    #[should_panic]
    fn test_maybe_panic() {
        // test_module内に移動すると、ひとつ上の名前空間の関数になるのでsuper::
        super::maybe_panic(true); //ok
        super::maybe_panic(false); // note: test did not panic as expected
    }
    
    #[test]
    #[should_panic(expected="フラグがtrueだぞい")]
    fn test_maybe_panic2() {
        super::maybe_panic(true); // expectedは部分一致で、正しいメッセージが panic と一緒に出たか判定できる。
    }

    // 関数名に　#[ignore]をつけると無視される。
    // cargo test test_maybe_panic2 のように関数指定でその関数だけテストになる。部分一致でいける。モジュール指定はないみたい。
    
}

/// 2つの関数を足し合わせるよ
/// ```
/// let result = rust_lesson::section13::sec13main::add(1. 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b:i32) -> i32 {
    a + b
}
/*
> cargo test
    Doc-tests rust_lesson

running 1 test
test src/lib.rs - say_goodbye (line 10) ... ok　 // これはsay_goodbyeだから違う

とドキュメントコメントの中もテストしてくれる。対象メソッドとドキュメントの内容の剥離を防げる。　-> なんか動かなかった

*/

