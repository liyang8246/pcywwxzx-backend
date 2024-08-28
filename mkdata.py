import os
import sqlite3


if not os.path.exists('data'):
    os.makedirs('data')
if not os.path.exists('data/db'):
    os.makedirs('data/db')
if not os.path.exists('data/certs'):
    os.makedirs('data/certs')

db_filename = 'data/db/data.sqlite'
if not os.path.exists(db_filename):
    conn = sqlite3.connect(db_filename)
    cursor = conn.cursor()

    create_table_sql = """
    CREATE TABLE IF NOT EXISTS issue (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        uid INTEGER NOT NULL,
        name TEXT NOT NULL,
        class TEXT NOT NULL,
        problem TEXT NOT NULL,
        reg_time TIMESTAMP NOT NULL,
        app_time TIMESTAMP NOT NULL,
        closed BOOLEAN NOT NULL,
        closed_time TIMESTAMP
    );
    """

    cursor.execute(create_table_sql)
    conn.commit()
    conn.close()
    
    print(f"Database '{db_filename}' created and 'issue' table initialized.")
else:
    print(f"Database '{db_filename}' already exists.")
