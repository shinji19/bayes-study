use bayes_study::bayes::Pmf;

fn main() {
    cookie();
    monty();
}

fn cookie() {
    println!("cookie");

    let mut pmf = Pmf::new();
    pmf.set(String::from("bowl1"), 0.5);
    pmf.set(String::from("bowl2"), 0.5);

    // 尤度
    pmf.multi(String::from("bowl1"), 3.0 / 4.0);
    pmf.multi(String::from("bowl2"), 1.0 / 2.0);

    pmf.normalize();

    println!("bowl1: {}", pmf.prob(String::from("bowl1")));
    println!("bowl2: {}", pmf.prob(String::from("bowl2")));
}

fn monty() {
    println!("monty");

    let mut pmf = Pmf::new();
    // それぞれの箱が選ばれる確率はどれも同じであることを表現
    pmf.set(String::from("A"), 1.0 / 3.0);
    pmf.set(String::from("B"), 1.0 / 3.0);
    pmf.set(String::from("C"), 1.0 / 3.0);

    // 尤度
    pmf.multi(String::from("A"), 0.5);
    pmf.multi(String::from("B"), 0.0);
    pmf.multi(String::from("C"), 1.0);

    pmf.normalize();

    println!("A: {}", pmf.prob(String::from("A")));
    println!("B: {}", pmf.prob(String::from("B")));
    println!("C: {}", pmf.prob(String::from("C")));
}
