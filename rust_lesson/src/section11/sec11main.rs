
pub fn lesson() {
    println!("");
    println!("##### section11だよ クロージャとイテレータ");

    closures();
    iterators();
    iterators_methods1();
}

fn closures() {
    println!("--------------- クロージャ");
/*
クロージャ
他の言語ではラムダ式と呼ばれたり。
*/
    // 1行で書く場合は{}を省略可能。戻り値をそのまま書いてセミコロン。
    let c1 = |x: i32| x + 1;
    println!("クロージャが入った変数c1実行 {}", c1(10)); // 11

    // 以下で m は「自由変数」。クロージャの外側の変数も使える。
    let m = 10;
    // c2が宣言された時点での自由変数mの値が閉じ込められるので、後で変えても変わらない。
    let c2 = |x: i32| x + m;
    println!("クロージャが入った変数c2実行 {}", c2(10)); // 10+10 で 20

    let m = 100;
    println!("シャドーイングでmの中身を上書き {}", m); // 100
    println!("クロージャが入った変数c2実行 {}", c2(10)); // 20 のまま

    let v = vec![1, 2, 3];
    // 引数、戻り値がないクロージャ
    let c3 = || {
        println!("vの値は{:?}", v)
    };
    c3(); // vの値は[1, 2, 3]

}

fn iterators() {
    println!("--------------- イテレーター");
    let v = vec![1,2,3,4,5];
    let v1_ite = v.iter();
    for x in v1_ite {
        println!("イテレーターv1_iteの中 {}", x); // 5回出力される
    }
    // イテレーターは必ずnextメソッドを実装している。Iteratorトレイト。
    let mut v2_ite = v.iter();
    println!("イテレーターv2_iteのnextを呼ぶ {:?}", v2_ite.next()); // Some(1)
    println!("イテレーターv2_iteのnextを呼ぶ {:?}", v2_ite.next()); // Some(2)
    println!("イテレーターv2_iteのnextを呼ぶ {:?}", v2_ite.next()); // Some(3)
    println!("イテレーターv2_iteのnextを呼ぶ {:?}", v2_ite.next()); // Some(4)
    println!("イテレーターv2_iteのnextを呼ぶ {:?}", v2_ite.next()); // Some(5)
    println!("イテレーターv2_iteのnextを呼ぶ {:?}", v2_ite.next()); // None
    // v．iter() で呼んだイテレーターは共有参照になる。
    // 可変参照にしたい場合は元のベクタもmut, 別メソッドで使用。
    let mut v2 = vec![1,2,3,4,5];
    let mut v2_ite_mutable = v2.iter_mut();
    println!("イテレーターv2_ite_mutableのnextを呼ぶ {:?}", v2_ite_mutable.next()); // Some(1)

    // 独自イテレーターを作って使う
    let counter = Counter {
        start: 1,
        end: 5,
    };
    for i in counter {
        println!("counterイテレーターの中身{}", i);
    }
    let mut counter2 = Counter {
        start: 1,
        end: 5,
    };
    println!("イテレーターcounterのnextを呼ぶ {:?}", counter2.next()); // Some(1)
    println!("イテレーターcounterのnextを呼ぶ {:?}", counter2.next()); // Some(2)
    println!("イテレーターcounterのnextを呼ぶ {:?}", counter2.next()); // Some(3)
    println!("イテレーターcounterのnextを呼ぶ {:?}", counter2.next()); // Some(4)
    println!("イテレーターcounterのnextを呼ぶ {:?}", counter2.next()); // None


}

struct Counter {
    start: u32,
    end: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

fn iterators_methods1() {
    println!("--------------- イテレーターのメソッド");
    let v = vec![1,2,3,4,5];
    // mapの引数にはクロージャを渡す。今回は引数を2倍にする
    let m = v.iter().map(|x| x * 2);
    for val in m {
        println!("イテレーターにmap関数を実行 {}", val); // 2,4,6,8,10の5回出力
    }

    let c: Vec<_> = v.iter().map(|x| x * 3).collect(); // 一気にベクタに変換する。型は推論してくれる。
    println!("イテレーターから作ったベクタ {:?}", c); //  [3, 6, 9, 12, 15]
    let c2: Vec<_> = v.iter().map(|x| x % 2 != 0).collect(); // 奇数かを判定
    println!("イテレーターから作ったベクタ {:?}", c2); //  [true, false, true, false, true]
    let f: Vec<_> = v.iter().filter(|x| *x % 2 != 0).collect(); // 奇数だけ取り出す。フィルタでは参照外しがいる
    println!("イテレーターから作ったベクタ {:?}", f); //  [1, 3, 5]

    let filter_fn = |x: &i32| x % 2 == 0; // 偶数だけtrueにするクロージャが入った変数
    let f2: Vec<_> = v.iter().map(filter_fn).collect(); //  これでもいけるかな
    println!("イテレーターから作ったベクタ {:?}", f2); //  [false, true, false, true, false]

    let filter_fn_4map = |x: &&i32| *x % 2 == 0; // 偶数だけ取り出すクロージャが入った変数
    let f3: Vec<_> = v.iter().filter(filter_fn_4map).collect(); // これでもいけるかな
    println!("イテレーターから作ったベクタ {:?}", f3); //  [2, 4]

    
}
