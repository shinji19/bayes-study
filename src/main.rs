use bayes_study::bayes::dice::Dice;
use bayes_study::bayes::monty::Monty;
use bayes_study::bayes::Pmf;

fn main() {
    cookie();
    monty();
    monty_suite();
    dice();
}

fn cookie() {
    println!("cookie");

    let mut pmf = Pmf::new();
    pmf.set("bowl1", 0.5);
    pmf.set("bowl2", 0.5);

    // 尤度
    pmf.multi("bowl1", 3.0 / 4.0);
    pmf.multi("bowl2", 1.0 / 2.0);

    pmf.normalize();

    println!("bowl1: {}", pmf.prob(&String::from("bowl1")));
    println!("bowl2: {}", pmf.prob(&String::from("bowl2")));
}

fn monty() {
    println!("monty");

    let mut pmf = Pmf::new();
    // それぞれの箱が選ばれる確率はどれも同じであることを表現
    pmf.set("A", 1.0 / 3.0);
    pmf.set("B", 1.0 / 3.0);
    pmf.set("C", 1.0 / 3.0);

    // 尤度
    pmf.multi("A", 0.5);
    pmf.multi("B", 0.0);
    pmf.multi("C", 1.0);

    pmf.normalize();

    println!("A: {}", pmf.prob(&String::from("A")));
    println!("B: {}", pmf.prob(&String::from("B")));
    println!("C: {}", pmf.prob(&String::from("C")));
}

fn monty_suite() {
    let hypos = vec![String::from("A"), String::from("B"), String::from("C")];
    let mut monty = Monty::new(&hypos);
    println!("monty suit\n{}", monty);
    println!("update b");
    monty.update("B");
    println!("monty suit\n{}", monty);
}

fn dice() {
    let hypos = vec![4, 6, 8, 12, 20];
    let mut dice = Dice::new(&hypos);
    println!("dice\n{}", dice);
    println!("update 6");
    dice.update(6);
    let ret = vec![6, 8, 7, 7, 5, 4];
    for r in ret {
        dice.update(r);
    }
    println!("update 6 8 7 7 5 4");
    println!("dice\n{}", dice);
}
