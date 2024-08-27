import os
import sqlite3

# 数据库文件名
db_filename = 'data/db/data.sqlite'

# 检查data.sqlite文件是否存在
if not os.path.exists(db_filename):
    # 创建并连接到数据库
    conn = sqlite3.connect(db_filename)
    cursor = conn.cursor()
    
    # 创建表的SQL语句
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
    
    # 执行创建表的SQL语句
    cursor.execute(create_table_sql)
    
    # 提交更改并关闭连接
    conn.commit()
    conn.close()
    print(f"Database '{db_filename}' created and 'issue' table initialized.")
else:
    print(f"Database '{db_filename}' already exists.")
