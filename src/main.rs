fn main() {
    println!("Hello, world!");
    let mut x = 5;
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {}", CONSTANT);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y : i32 = 5;

    let y :i32 = y + 1; // 6

    // シャドーイング
    {
        let z: i32 = 5;
        let y: i32 = y * 2; // 12
        println!("The value of y in the inner scope is : {}", y);
        println!("The value of z is : {}", z);
    }
    // println!("The value of z is : {}", z);
    println!("The Value of y is : {}", y);

    let some_strings: &str = "aaa";
    println!("The value of spaces is : {}", some_strings);

    let some_strings: usize = some_strings.len();
    println!("The value of spaces is : {}", some_strings);

    // Rustのデータ型
    // addition
    let sum = 5 + 10;
    println!("The Value of sum is : {}", sum);

    // subtration
    let diffrence = 99.5 - 4.3;
    println!("The Value of diffrence is : {}", diffrence);

    // multiplication
    let product = 4 * 30;
    println!("The Value of product is : {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The Value of quotient is : {}", quotient);

    let floored = 2 / 3;
    println!("The Value of floored is : {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The Value of remainder is : {}", remainder);

    {
        let x: usize = 6;
        let y: f64 = 1.5;
        // let z = x / y;
        let z = (x as f64) / y;
        println!("The Value of z is : {}", z);
        
        // 論理値型
        let t = true;
        println!("The Value of t is : {}", t);

        let f: bool = false; // with explicit type annotation
        println!("The Value of t is : {}", f);

        // 文字型
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';    //ハート目の猫
        println!("The Value of t is : {}", c);
        println!("The Value of t is : {}", z);
        println!("The Value of t is : {}", f);
        println!("The Value of t is : {}", heart_eyed_cat);

        // タプル型
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {}", y);

        let x: (i32, f64, u8) = (500, 6.4, 1);
        
        let five_hundred = x.0;
        
        let six_point_four = x.1;
        
        let one = x.2;

        
        // 配列型
        let a = [1, 2, 3, 4, 5];
        
        // 配列の長さが固定（スタックに）
        // 長さが変えたいときはベクターを使う
        let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
        
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let a = [3; 5];

        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
        println!("The Value of t is : {}", first);
        println!("The Value of t is : {}", second);

        // 配列の範囲外を指定するとパニックする。
        // ※パニックとは、プログラムがエラーで終了したことを表すRust用語。
    }
    // 関数の話
    // fn main() 
    {
        println!("Hello, world!");
    
        another_function();
    }

}


fn another_function() {
    println!("Another function.");  // 別の関数
    main1();
}

fn main1() {
    another_function1(5);
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
    main2();
}

// 引数がある場合
fn main2() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
    main3();
}

fn main3() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // y = 4
    main4();
}

// 戻り値のある関数

fn five() -> i32 {
    5
}

fn main4() {
    println!("戻り値のある関数の表記");
    let x = five();

    println!("The value of x is: {}", x);
    main5();
}

fn main5() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
    main6();
}

fn plus_one(x: i32) -> i32 {
    x + 1 //; ⇐ この「セミコロン」を文末につけるとreturnされなくなる。
}

// 制御フロー
fn main6() {
    println!("if文について");
    let number = 3;

    if number < 5 {
        println!("condition was true");       // 条件は真でした
    } else {
        println!("condition was false");      // 条件は偽でした
    }

    // 条件式は、bool型でなければならない

    // else ifで複数の条件を扱う
    // ifとelseを組み合わせてelse if式にすることで複数の条件を持たせることも。

    {
        let number = 6;

        if number % 4 == 0 {
            // 数値は4で割り切れます
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            // 数値は3で割り切れます
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            // 数値は2で割り切れます
            println!("number is divisible by 2");
        } else {
            // 数値は4、3、2で割り切れません
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    { // let文内でif文を使う。
        let condition = true;
        let number = if condition { 5 } else { 6 };
    
        // numberの値は、{}です
        println!("The value of number is: {}", number);
    }

    // { //同じ方がある必要がある.
    //     let condition = true;
    
    //     let number = if condition { 5 } else { "six" }; //同じ方がある必要がある.
    
    //     println!("The value of number is: {}", number);
    // }

    // loppでの繰り返し - loop - while - for
    {
        // loop (無限ループ)
        // loop {
        //     println!("again!");   // また
        // }

        // while
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;
    
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        
        }
        println!("End count = {}", count);
    }

    {
        let mut number = 3;
    
        while number != 0 {
            println!("{}!", number);
    
            number -= 1;
        }
    
        // 発射！
        println!("LIFTOFF!!!");
    }

    // forでコレクションを覗き見る
    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
    
        while index < 5 {
            // 値は{}です
            println!("the value is: {}", a[index]);
    
            index += 1;
        }
    }

    {
        let a = [10, 20, 30, 40, 50];
    
        for element in a {
            println!("the value is: {}", element);
        }
    }

    {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }

    // ・所有権
    // 所有権規則
    // --Rustの書く値は、所有者と呼ばれる変数と対応している。
    // --いかなるときも所有者がスコープから外れたら値は破壊される。

    // 変数スコープ
    { 
        let s = String::from("hello"); // sはここから有効になる
        println!("the value is: {}", s);
        // sで作業をする
    } // このスコープはここでおしまい。sは
    // もう有効ではない

    // string型 → ヒープで保持される。 ブロックから抜けるタイミングで保持が解除される。
    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
        println!("{}", s); // これは`hello, world!`と出力する
    }
    // OSにメモリを返す。

    //　変数とデータの相互作用法: ムーブ



}

