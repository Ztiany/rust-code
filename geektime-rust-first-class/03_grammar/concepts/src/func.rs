//====================================================================
//演示函数的定义
//====================================================================

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 { value * value }

fn cube(value: i32) -> i32 { value * value * value }

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    //没有定义返回类型，且组后一个语句使用 ; 结束，则不当作返回值返回。
    3.1415926;
}

pub(crate) fn play_func() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    let is_unit2 = {
        pi();
    };

    let is_pi = pi();
    let is_unit1 = not_pi();

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}
