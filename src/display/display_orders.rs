use super::*;
use cryptotrader;
use cryptotrader::models::*;

pub fn table(orders: Vec<Order>) -> String {
    orders
        .iter()
        .map(table_row)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn table_row(order: &Order) -> String {
    format!(
        "{:<normal_width$}{:<normal_width$}{:<normal_width$}{:<wide_width$.8}{:normal_width$}",
        order.symbol.yellow(),
        format!("{}", order.order_type),
        order.qty,
        order.price,
        order.time.format("%Y-%m-%d %H:%M").to_string(),
        normal_width = NORMAL_COLUMN_WIDTH,
        wide_width = WIDE_COLUMN_WIDTH,
    )
}
