-- Create a table to store users
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,
    username TEXT NOT NULL,
    full_name TEXT,
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create a table to store dialogues (optional if you use a dialogue system)
CREATE TABLE IF NOT EXISTS dialogues (
    dialogue_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    state TEXT NOT NULL,
    data TEXT,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);

-- Insert some dummy users
INSERT INTO users (user_id, username, full_name) VALUES (123456789, 'john_doe', 'John Doe');
INSERT INTO users (user_id, username, full_name) VALUES (987654321, 'jane_smith', 'Jane Smith');
INSERT INTO users (user_id, username, full_name) VALUES (111222333, 'alice_williams', 'Alice Williams');

-- Insert dummy dialogues for testing
INSERT INTO dialogues (user_id, state, data) VALUES (123456789, 'waiting_for_input', '{"step": 1}');
INSERT INTO dialogues (user_id, state, data) VALUES (987654321, 'awaiting_approval', '{"step": 2}');
INSERT INTO dialogues (user_id, state, data) VALUES (111222333, 'completed', '{"step": 3}');
