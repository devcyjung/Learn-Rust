fn main() {
    let mut months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let m1 = &months;
    print_months(m1);
    let m2 = &mut months;
    print_months_reversed(m2);
    let m3 = &months;
    print_months(m3);
}

fn print_months(months: &[&str; 12]) {
    for month in months {
        print!("{} ", month)
    }
    println!()
}

fn print_months_reversed(months: &mut [&str; 12]) {
    months.reverse();
    print_months(&months);
}
