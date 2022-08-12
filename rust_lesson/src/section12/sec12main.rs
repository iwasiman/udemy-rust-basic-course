use std::collections::{VecDeque, BinaryHeap};


pub fn lesson() {
    println!("");
    println!("##### section12だよ コレクション");
    // ヒープに格納されるのでサイズが変更できる。

    vectors_part1();
    vectors_part2();
    ques();
    maps();
    sets();
}

fn vectors_part1() {
    println!("--------------- ベクタ Part1");
    let v1 = vec!["Rust", "Python", "Java"];
    println!("ベクタ v1 {:?}", v1);
    println!(".as_ptr() {:?}", v1.as_ptr());
    println!(".len() {:?}", v1.len()); //3
    println!(".capacity() {:?}", v1.capacity()); // 3

    println!("0番目の要素を直接 {:?}", &v1[0]); // 存在しないインデックスだとパニックが発生してしまう
    println!("0番目の要素をget {:?}", v1.get(0)); // 戻り値はOption型で安全
    
    let mut v3 = vec!["Rust", "Python", "Java"];
    v3.push("PHP");
    println!("push後のベクタ v3 {:?}", v3);
    let val = v3.pop();
    println!("popした値 {:?}", val);
    println!("pop後のベクタ v3 {:?}", v3);

    v3.insert(1, "PHP"); // 途中に挿入もできる
    println!("index:1 に insert後のベクタ v3 {:?}", v3);
    v3.remove(2); // 途中の削除もできる
    println!("index:2 を remove後のベクタ v3 {:?}", v3);


}

fn vectors_part2() {
    println!("--------------- ベクタ Part2");
    let v1 = vec!["Rust", "Python", "Java"];
    let v2 = vec!["PHP", "Go"];
    let v3 = [v1, v2].concat();
    println!("concatしたベクタ v3 {:?}", v3); // ["Rust", "Python", "Java", "PHP", "Go"]

    let (v4, v5) = v3.split_at(2); // 0〜 mid-1, mid 〜 終端 で分割される
    println!("mid:2で分割したベクタ {:?}", v4); // ["Rust", "Python"]
    println!("mid:2で分割したベクタ {:?}", v5); // ["Java", "PHP", "Go"]

    let mut v6 = vec![3,6,1,7,2];
    v6.sort();
    println!("ベクタを昇順ソートする {:?}", v6); // [1, 2, 3, 6, 7]
    v6.reverse();
    println!("ベクタを降順ソートする {:?}", v6); // [7, 6, 3, 2, 1]

    #[derive(Debug)]
    struct Suu {
        val1: i32,
        val2: i32,
    }
    let mut v7 = vec![
        Suu {val1:3, val2: 1},
        Suu {val1:2, val2: 2},
        Suu {val1:1, val2: 3},
    ];
    v7.sort_by_key(|s| s.val1);
    println!("クロージャでval1でソートするようにした構造体Suuのベクタ {:?}", v7);
    println!("{:?}", v7[0].val2); // 警告避け

    let v8 = vec![1,2,3,4,5];
    println!("contains()でベクタv8に1が含まれているか {}", v8.contains(&1)); // true 引数は参照で渡す必要があるのに注意
    println!("contains()でベクタv8に99が含まれているか {}", v8.contains(&99)); // false 引数は参照で渡す必要があるのに注意

    // ベクタ内を検索して何番目にあるかは関数が用意されていない。イテレーターに変換、クロージャを使うとできる。下は5は何番目かを判定。
    let where_is_5 = v8.iter().position(|x| *x == 5); // *をつけて参照外しがいる
    println!("5はどこかな {:?}", where_is_5); // Some(4)

}

fn ques() {
    println!("--------------- キュー");
    // キューはベクタと違いFIFO。

    let mut q: VecDeque<i32> = VecDeque::new(); // 最初が空の場合
    //let mut q1: VecDeque<i32> = VecDeque::from(vec![1,2,3]); // 値を入れた場合
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    println!("キューに値を追加 {:?}", q); // [1, 2, 3]
    println!("キューの最初を.pop_front()で取り出す {:?}", q.pop_front()); // Some(1)
    println!("取り出された残り {:?}", q); // [2, 3]

    // バイナリヒープは最大値が常に最初に格納される こういうのを「優先度つきキュー」と呼ぶ
    let mut bh:BinaryHeap<i32> = BinaryHeap::new();
    bh.push(1);
    bh.push(10);
    bh.push(20);
    bh.push(5);
    println!("バイナリヒープの中身 {:?}", bh); // [20, 5, 10, 1] // 昇順というわけではないようだ。謎。
    println!("バイナリヒープをpopすると先頭の最大値が取り出される {:?}", bh.pop()); // Some(20)
    println!("バイナリヒープの残り {:?}", bh); // [10, 5, 1]
    // 優先度付きのタスクキューを常に先頭に配置したりする時に使う。オーダートレイトを実装して特定の値で優先度付できるようにしたり。

}

use std::collections::HashMap;
fn maps() {
    println!("--------------- マップ");
    // ハッシュ、連想配列、ディクショナリのこと。

    let mut map: HashMap<&str, i32> = HashMap::new(); // 動画と違い、型指定必須
    map.insert("Japan", 11); // VSCodeの機能でk： v: が表示上は補完される
    map.insert("USA", 3);
    map.insert("China", 1);
    map.insert("India", 2);
    println!("mapを表示するよ {:?}", map); // {"Japan": 11, "USA": 3, "India": 2, "China": 1}
    map.insert("Japan", 10); // 上書きされる
    println!("上書き後のmapを表示するよ {:?}", map); // {{"USA": 3, "India": 2, "China": 1, "Japan": 10} 最後に来る
    println!("get()で1要素を取得するよ {:?}", map.get("USA")); // Some(3)
    println!("remove()でIndiaを削除するよ {:?}", map.remove("India")); // Some(2)
    println!("削除後のmapを表示するよ {:?}", map); // {"USA": 3, "China": 1, "Japan": 10}

}
use std::collections::HashSet;
fn sets() {
    println!("--------------- セット");
    // 型に対する集合
    let mut set1: HashSet<i32> = HashSet::new(); // ジェネリクスの型指定必須
    set1.insert(1);
    set1.insert(1);
    set1.insert(1);
    println!("セットを表示するよ {:?}", set1); // {1}
    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    println!("セットを表示するよ {:?}", set1); // {3, 1, 4, 2} 順番は保証されない
    println!("含まれているかはcontainsで 2はいるかな {:?}", set1.contains(&2)); // true ここでも参照を渡す
    println!("remove()で削除 2を消す {:?}", set1.remove(&2)); // true

    let mut set2: HashSet<i32> = HashSet::new();
    set2.insert(11);
    set2.insert(22);
    set2.insert(33);
    set2.insert(1);
    let set3: HashSet<i32> = &set1 | &set2;
    println!("&set1 | &set2 で和集合のセット {:?}", set3); // {4, 3, 1, 11, 22, 33}
    let set4: HashSet<i32> = &set1 & &set2;
    println!("&set1 & &set2 で積集合のセット {:?}", set4); // {1} 両方にあるものだけ
    let set5: HashSet<i32> = &set1 - &set2;
    println!("&set1 - &set2 で差集合のセット {:?}", set5); // {3, 4} set1の{1, 3, 4}から1が惹かれる
    let set6: HashSet<i32> = &set1 ^ &set2;
    println!("&set1 ^ &set2 で排他的論理和集合のセット {:?}", set6); // {11, 33, 3, 4, 22} 片方にしかないものだけ



}
