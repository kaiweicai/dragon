fn main() {
    println!("Hello world!");
    let init_amount: u64 = 200_000; //初始资金20w。
    println!("初始资金:{:?}", init_amount);
    let create_percent:u64 = 40;
    println!("----------------------------------每天增长:4%");
    let mut amount_percent4 = init_amount;
    for i in 1..24*4 {
        if (i-1) %24 ==0 {
            println!("-------第{:?}个月----------", i/24+1);
        }
        //每天增长4%
        amount_percent4 = amount_percent4 + amount_percent4 * create_percent / 1000;
        println!("第{:?}天,当前金额：{:?}", i,amount_percent4);
    }

    println!("");
    let create_percent:u64 = 35;
    println!("----------------------------------每天增长:3.5%");
    let mut amount_percent4 = init_amount;
    for i in 1..24*4 {
        if (i-1) %24 ==0 {
            println!("-------第{:?}个月----------", i/24+1);
        }
        //每天增长4%
        amount_percent4 = amount_percent4 + amount_percent4 * create_percent / 1000;
        println!("第{:?}天,当前金额：{:?}", i,amount_percent4);
    }

}
