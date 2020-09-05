use apg::*;

fn main() {
    test_product();    
    test_coproduct();
}

fn test_product() {
    let apg1 = make_named_one("false");
    let apg2 = make_named_one("true");
    let apg3 = product_of_apgs(&apg1, &apg2);
    println!("{:?}", apg3);

    let apg4 = make_named_one("small");
    let apg5 = make_named_one("medium");
    let apg6 = make_named_one("large");
    let apg7 = product_of_apgs(&product_of_apgs(&apg4, &apg5), &apg6);
    println!("{:?}", apg7);
}

fn test_coproduct() {
    let apg1 = make_named_one("false");
    let apg2 = make_named_one("true");
    let apg3 = coproduct_of_apgs(&apg1, &apg2);
    println!("{:?}", apg3);
}

fn _sum_books_price() {
    let prices = vec![
        1881,
        1737,
        2106,
        1962,
        72,
        2052,
        1602,
        3762,
        837,
        1510,
        2268,
        720,
        2392,
    ];
    let sum = prices.iter().fold(0, |sum, p| sum + p);
    println!("{:?}", sum);
}