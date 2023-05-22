//電卓
fn main(){
    //変数宣言
    let mut num1 = 0;
    let mut num2 = 0;
    let mut ope = String::new();
    let mut result = 0;

    //入力
    println!("数値を入力してください");
    let mut input_num1 = String::new();
    std::io::stdin().read_line(&mut input_num1).expect("Failed to read line");
    num1 = input_num1.trim().parse().expect("Failed to parse");

    println!("演算子を入力してください");
    let mut input_ope = String::new();
    std::io::stdin().read_line(&mut input_ope).expect("Failed to read line");
    ope = input_ope.trim().to_string();

    println!("数値を入力してください");
    let mut input_num2 = String::new();
    std::io::stdin().read_line(&mut input_num2).expect("Failed to read line");
    num2 = input_num2.trim().parse().expect("Failed to parse");

    //計算
    if ope == "+"{
        result = num1 + num2;
    }else if ope == "-"{
        result = num1 - num2;
    }else if ope == "*"{
        result = num1 * num2;
    }else if ope == "/"{
        result = num1 / num2;
    }else{
        println!("演算子が不正です");
    }

    //出力
    println!("{} {} {} = {}", num1, ope, num2, result);

    
}