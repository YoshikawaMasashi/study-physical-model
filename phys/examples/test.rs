use phys;

fn main() {
    let fdmrunner = phys::finite_difference_method::FDMRunner::new(
        (300, 300),
        1.0,
        0.1,
        0.1,
        (100, 100));
    
    let ret = fdmrunner.run(1000);
    println!("test");
}
