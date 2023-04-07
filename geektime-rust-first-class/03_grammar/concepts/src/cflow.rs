//====================================================================
//通过斐波拉契的生成演示控制流
//====================================================================

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    println!("fib_loop started");
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
        if i >= n { break; }
    }
    println!("fib_loop ended");
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    println!("fib_while started");
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
    }
    println!("fib_while ended");
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    println!("fib_for started");
    /* 遍历：[2,n) */
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
    println!("fib_for ended");
}

pub(crate) fn play_cflow() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}