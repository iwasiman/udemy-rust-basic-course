fn main() {
    println!("FizzBuzzだよ");
    fizzbuzz(30);
    fizzbuzz_for_match(30);
    fizzbuzz_tupple(30);

}

fn fizzbuzz(end :i32) {
    println!("----- while バージョン");
    let mut x = 1;
    while x <= end {
        if x % 3 == 0 && x % 5 == 0 {
            println!("3と5の倍数なのでFizzBuzz!:: {}", x);
        } else if x % 3 == 0 {
            println!("3の倍数なのでFizz:: {}", x);
        } else if x % 5 == 0 {
            println!("5の倍数なのでBuzz:: {}", x);
        } else {
            println!("どれでもないよ:: {}", x);
        }
        x += 1;
    }
    println!("");
}

fn fizzbuzz_for_match(end :i32) {
    println!("----- for と match バージョン");
    let r = 1..=end; // r = 1..end と書くだけでよい
    for x in r {
        // 15で割った余りで分岐させている。最後の_が抜けているとコンパイルエラーで教えてくれる。
        match x % 15 {
            0 => println!("3と5の倍数なのでFizzBuzz!:: {}", x),
            3 | 6 | 9 | 12 => println!("3の倍数なのでFizz:: {}", x),
            5 | 10 => println!("5の倍数なのでBuzz:: {}", x),
            _ => println!("どれでもないよ:: {}", x),
        }
    }
    println!("");
}

fn fizzbuzz_tupple(end :i32) {
    println!("----- tupple バージョン");
    let r = 1..=end; // r = 1..end と書くだけでよい
    for x in r {
        match (x % 3, x% 5) {
            (0, 0) => println!("3と5の倍数なのでFizzBuzz!:: {}", x),
            (0, _ ) => println!("3の倍数なのでFizz:: {}", x), //5で割っても余りなしでここにマッチするが、前の行でマッチ済みなので大丈夫
            (_, 0) => println!("5の倍数なのでBuzz:: {}", x),
            _ => println!("どれでもないよ:: {}", x),
        }
    }
    println!("");
}
