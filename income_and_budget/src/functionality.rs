use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use rust_decimal_macros::dec;
use rust_decimal::Decimal;

fn add_portions(portion_name: &str, portion_amount: &str) -> HashMap<String, Decimal> {
    let mut portion_name_buffer = String::new();
    let mut portion_amount_buffer = String::new();
    let mut add_portion_buffer = String::new();
    let mut map = HashMap::new();

    println!("Enter the amount of {}s you would like to have:", &portion_name);

    io::stdin()
        .read_line(&mut add_portion_buffer)
        .expect("Failed to readline");

    let mut add_portion_buffer: u8 = add_portion_buffer.trim().parse().expect("Failed to convert to integer");

    while add_portion_buffer > 0 {
        portion_name_buffer.clear();
        portion_amount_buffer.clear();

        println!("Please enter the name of the {} you would like to add:", &portion_name);
        io::stdin()
            .read_line(&mut portion_name_buffer)
            .expect("Failed to readline");

        let portion_name_buffer = portion_name_buffer.trim().to_string();

        println!("please enter the {} for {}", &portion_amount, &portion_name_buffer);
        io::stdin()
            .read_line(&mut portion_amount_buffer)
            .expect("Failed to readline");

        let portion_amount_buffer: Decimal = portion_amount_buffer.trim()
            .parse()
            .expect("Failed to convert");

        map.insert(portion_name_buffer.clone(), portion_amount_buffer);

        add_portion_buffer -= 1;
    }

    map
}

fn add_budget_portions() -> HashMap<String, Decimal> {
    add_portions("budget portion", "budget percentage")
}

fn calculate_income_by_budget(income: Vec<Decimal>, budget_portions: HashMap<String, Decimal>) {
    for (budget_portion_name, budget_portion_value) in &budget_portions {
        for my_income in &income {
            let calculation = my_income * budget_portion_value / dec!(100.0);

            println!("{}: {}", budget_portion_name.as_str(), calculation);
        }
    }
}

// enter paychecks here
fn income() -> Vec<Decimal> {
    let income = vec![
        Decimal::from_str("1000.56").unwrap(),
    ];
    income
}

struct Expenses {
    name: String,
    price: Decimal,
}

// keep track of your expenses here
fn expenses() -> Vec<Expenses> {
    let expenses = vec![
        Expenses { name: "bill".to_string(),
            price: dec!(100)},
    ];
    expenses
}

fn calculate_total_expenses(expenses: Vec<Expenses>) {

    let mut cost = dec!(0);
    for expense in &expenses {
        println!("Purchase Name: {} \nItem Cost: {}\n", expense.name, expense.price);
        cost += expense.price;
    }
    println!("Total Spent: {}", cost);
}

fn calculate_total_income(income: Vec<Decimal>) -> Decimal {
    let mut total_income: Decimal = dec!(0);
    for my_income in &income {
        total_income += my_income
    }
    total_income
}


// takes the total income and subtracts the expenses
fn calculate_total_income_with_expenses(expenses: Vec<Expenses>) -> Decimal {
    let total_income = calculate_total_income(income());
    let mut total_expense_cost = dec!(0);
    for expense in &expenses {
        total_expense_cost += expense.price;
    }
    let calculation = total_income - total_expense_cost;
    calculation
}

// calculates total income with expenses and by a budget system
fn calculate_overall_income_by_budget() {
    let calculation = calculate_total_income_with_expenses(expenses());
    let income = vec![calculation];
    let budget_system = add_budget_portions();

    calculate_income_by_budget(income, budget_system);
}

// a function that calculates the total amount of money for each budget portion
fn calculate_each_budget_portion_money() -> HashMap<String, Decimal> {
    println!("Please enter your total income: ");
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to readline");


    let total_income: Decimal = buffer.trim().parse().expect("Should be a number");

    println!("Please add budget portions to divide up your total income");
    let budget_system = add_budget_portions();
    let mut map = HashMap::new();

    for (budget_name, budget_percentage) in budget_system {
        map.insert(budget_name, total_income * budget_percentage / dec!(100));
    }

    map
}


fn view_income_in_each_budget_portion() {
    let portions = calculate_each_budget_portion_money();

    for (budget_name, income) in &portions {
        println!("Budget name: {} Total amount: {}", budget_name, income);
    }
}

#[derive(Debug)]
struct Item {
    item_name: String,
    item_cost: Decimal,
    budget_name: String,
    budget_percentage: Decimal,
}

fn add_items_for_each_budget() -> Vec<Item> {
    let mut items = vec![];
    let mut total_item_buffer = String::new();
    let mut item_name_buffer = String::new();
    let mut item_cost_buffer =String::new();

    println!("Please add a budget for new items");
    println!("Note: the budget percentage will apply to all items!");

    let budget_system = add_budget_portions();



    for (budget_name, budget_percentage) in &budget_system {
        total_item_buffer.clear();
        println!("Please enter the total amount of items you wish to add for {}", budget_name);
        io::stdin()
            .read_line(&mut total_item_buffer)
            .expect("Failed to readline");

        let mut total_item_buffer: u8 = total_item_buffer.trim().parse().expect("should be a number");

        while total_item_buffer > 0 {
            item_name_buffer.clear();
            item_cost_buffer.clear();

            let budget_portion_name = budget_name.to_string();
            let budget_portion_percentage = budget_percentage.to_owned();

            println!("Reminder! the budget percentage is: {}", budget_percentage);
            println!("please enter the name of the item you wish to add for {}", budget_name);

            io::stdin()
                .read_line(&mut item_name_buffer)
                .expect("Failed to readline");

            let item_name_buffer = item_name_buffer.trim().to_string();

            println!("Please enter the cost of the item: ");

            io::stdin()
                .read_line(&mut item_cost_buffer)
                .expect("Failed to readline");

            let item_cost_buffer: Decimal = item_cost_buffer.trim().parse().expect("Should be a number!");

            items.push(Item {
                item_name: item_name_buffer,
                item_cost: item_cost_buffer,
                budget_name: budget_portion_name,
                budget_percentage: budget_portion_percentage,
            });

            total_item_buffer -= 1;
        }
    }

    items
}

fn calculate_item_by_budget() {
    let divide_income = calculate_each_budget_portion_money();
    let items = add_items_for_each_budget();

    for (budget_name, mut income) in divide_income {
        for item in &items {
            if budget_name == item.budget_name {
                let my_income = income * item.budget_percentage / dec!(100);
                println!("item name: {} item cost: {} amount allocated: {}", item.item_name, item.item_cost, my_income);

                income -= my_income;
            }
        }
    }
}

enum Choices {
    ViewOverallIncomeByBudget,
    ViewTotalExpenses,
    ViewTotalIncome,
    ViewEachBudgetPortionIncome,
    ViewItemsByBudgetSystem,
}

fn prompt_user() -> Result<Choices, String> {
    let intro = format!("Welcome to the income and budget app! Please type in the associated number for the task you wish to perform!
              1) View overall income by budget
              2) View total expenses
              3) View total income
              4) View total amount of money for each budget portion
              5) View items by a budget system
              ");
    println!("{}", intro);

    let mut choice_buffer = String::new();

    io::stdin()
        .read_line(&mut choice_buffer)
        .expect("Failed to readline");

    let choice_buffer: u8 = choice_buffer.trim().parse().expect("Invalid input!");

    let choice: Result<Choices, String>;

    match choice_buffer {
        1 => choice = Ok(Choices::ViewOverallIncomeByBudget),
        2 => choice = Ok(Choices::ViewTotalExpenses),
        3 => choice = Ok(Choices::ViewTotalIncome),
        4 => choice = Ok(Choices::ViewEachBudgetPortionIncome),
        5 => choice = Ok(Choices::ViewItemsByBudgetSystem),
        _ => choice = Err(String::from("Invalid Choice!")),
    }

    choice
}

pub fn run() {
    let choice = prompt_user();
    match choice {
        Ok(Choices::ViewOverallIncomeByBudget) => calculate_overall_income_by_budget(),
        Ok(Choices::ViewTotalIncome) => println!("Total Income: {}", calculate_total_income_with_expenses(expenses())),
        Ok(Choices::ViewTotalExpenses) => calculate_total_expenses(expenses()),
        Ok(Choices::ViewEachBudgetPortionIncome) => view_income_in_each_budget_portion(),
        Ok(Choices::ViewItemsByBudgetSystem) => calculate_item_by_budget(),
        Err(e) => println!("{}", e),
    }
}
