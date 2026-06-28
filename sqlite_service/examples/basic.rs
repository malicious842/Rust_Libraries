//sql_serviceを使用する場合このコードを参考にしてください
use sqlite_service::SqlteDB;

fn main() {
    let db = SqlteDB::open("examples/test.db").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS prefecture (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            population INTEGER,
            capital TEXT
);"
    ).unwrap();

    //query_as_stringは[string]型で返します
    let result_text =db.query_as_string(
        "SELECT * FROM prefecture;"
        ).unwrap();

        for row01 in result_text{
            println!("{:?}",row01)
        }
    
    //query_as_valueは型を保持したまま返します
    let result_value= db.query_as_value(
        "SELECT * FROM prefecture;"
    ).unwrap();


    //特定の行が必要な場合、[.iter().take()]を使用してください
    for row in result_value.iter().take(2)
    {
        println!("{:?}", row);
    }


    println!("成功");


}