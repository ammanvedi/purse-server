CREATE TABLE IF NOT EXISTS account_holder(
  id INTEGER PRIMARY KEY,
  username TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  access_token TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS categories(
  id TEXT PRIMARY KEY,
  description TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS bank_account(
  id TEXT PRIMARY KEY,
  balance_current REAL NOT NULL,
  balance_available REAL NOT NULL,
  account_name TEXT NOT NULL,
  account_type TEXT NOT NULL,
  iso_currency TEXT NOT NULL,
  account_holder_id INTEGER NOT NULL,
  FOREIGN KEY (account_holder_id) REFERENCES account_holder (id) ON DELETE CASCADE ON UPDATE NO ACTION
);
CREATE TABLE IF NOT EXISTS account_transactions(
  id TEXT PRIMARY KEY,
  amount REAL NOT NULL,
  utc_timestamp INTEGER NOT NULL,
  iso_currency TEXT,
  name TEXT,
  category TEXT,
  pending INTEGER NOT NULL,
  bank_account_id TEXT NOT NULL,
  pending_transaction_id TEXT,
  transfer_transaction_id TEXT,
  FOREIGN KEY (bank_account_id) REFERENCES bank_account (id) ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (category) REFERENCES categories (id) ON DELETE
  SET
    NULL ON UPDATE NO ACTION,
    FOREIGN KEY (pending_transaction_id) REFERENCES account_transactions (id) ON DELETE
  SET
    NULL ON UPDATE NO ACTION,
    FOREIGN KEY (transfer_transaction_id) REFERENCES account_transactions (id) ON DELETE
  SET
    NULL ON UPDATE NO ACTION
);
CREATE TABLE IF NOT EXISTS expense(
  paused INTEGER NOT NULL,
  id INTEGER PRIMARY KEY,
  frequency_type TEXT NOT NULL,
  frequency_date INTEGER,
  name TEXT NOT NULL,
  description TEXT,
  utc_timestamp_created INTEGER NOT NULL,
  price_bound_lower FLOAT NOT NULL,
  price_bound_upper FLOAT NOT NULL
);
CREATE TABLE IF NOT EXISTS expense_transaction(
  expense_id INTEGER NOT NULL,
  transaction_id TEXT NOT NULL,
  PRIMARY KEY (expense_id, transaction_id),
  FOREIGN KEY (expense_id) REFERENCES expense (id) ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (transaction_id) REFERENCES account_transactions (id) ON DELETE CASCADE ON UPDATE NO ACTION
)