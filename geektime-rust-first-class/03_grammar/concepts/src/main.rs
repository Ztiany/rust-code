mod func;
mod stru;
mod cflow;
mod utest;

fn main() {
    print_args();
    func::play_func();
    stru::play_stru();
    cflow::play_cflow();
}

/** 打印程序参数 */
fn print_args() {
    let mut index = 0;
    for arg in std::env::args() {
        println!("arg-{}: {}", index, arg);
        index = index + 1;
    }
}

