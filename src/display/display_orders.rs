use super::*;
use crate::display;
use cryptotrader;
use cryptotrader::models::*;
use prettytable::{cell, row, Row, Table};

pub fn table(orders: Vec<Order>) -> String {
    let mut table = Table::new();
    table.set_format(table_format());
    table.set_titles(row!(
        "PAIR",
        "ORDER_TYPE",
        "SIDE",
        "SIZE",
        "QTY",
        "PRICE",
        "TIME"
    ));

    for order in orders {
        table.add_row(table_row(&order));
    }

    format!("{}", table)
}

pub fn table_row(order: &Order) -> Row {
    row!(
        format!("{}", order.pair).yellow(),
        format!("{}", order.order_type),
        display::trade_type::colored(order.trade_type),
        format!("{:.2}", order.qty * order.price),
        order.qty,
        format!(
            "{:.8} ({:.2}%)",
            order.price,
            order.price_difference_as_percent()
        ),
        order.time.format("%Y-%m-%d %H:%M").to_string(),
    )
}
