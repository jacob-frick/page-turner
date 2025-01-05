CREATE TABLE user_book_list (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    book_id UUID REFERENCES books(id) ON DELETE CASCADE,
    status_id UUID REFERENCES book_statuses(id) ON DELETE CASCADE, -- Foreign key to BookStatus table
    PRIMARY KEY (user_id, book_id)
);