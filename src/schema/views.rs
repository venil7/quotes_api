table! {
  users_holdings (user_id) {
      user_id -> Int4,
      ticker -> Varchar,
      amount -> Float8,
  }
}
