use std::io;

fn main() {
    loop {
        let mut number_of_terms = String::new();

        println!("Enter the number of terms");
        io::stdin()
            .read_line(&mut number_of_terms)
            .expect("error");

        let number_of_terms: u16 = number_of_terms.trim()
            .parse()
            .expect("not a number");

        println!("Fibonacci series");

        fibonacci_calc(number_of_terms, 0, 1);
        println!(" ");
    }
}

fn fibonacci_calc(number_of_therms: u16, mut current_term: u128, mut next_term: u128) {
    if number_of_therms != 0 {
        print!("{} ", current_term);
        let third_term = current_term + next_term;
        current_term = next_term;
        next_term = third_term;
        fibonacci_calc(number_of_therms - 1, current_term, next_term);
    }
}
