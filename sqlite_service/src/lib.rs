
//SQLiteを利用するためのサービスはこちらで提供いたします。
use rusqlite::{Connection, Result};

//SQLiteが持っている型をRust側で表現するための型です
use rusqlite::types::Value;


//SQLiteへの接続を保持するための構造体です
pub struct SqlteDB{
    conn: Connection,
}
impl SqlteDB {
    //SQLiteファイルを開いて、その接続を持った SqlteDB を作る関数
    //SQLiteファイルが無ければ作成します
    pub fn open(path: &str)-> Result<Self>
    {
        Ok(Self {conn:Connection::open(path)?,})
    }


    //SQL文を実行するための関数
    pub fn execute(&self,sql: &str)-> Result<()>
    {
        self.conn.execute(sql, [])?;
        Ok(())
    }

    //SQLのSELECT文を実行して、複数の文字列を全部 Vec<String> にして返す関数
    //関数：[query_all]はすべて[String]型になります
    pub fn query_as_string(&self,sql: &str,) -> Result<Vec<Vec<String>>>
    {
        //sql文をここでコンパイルしています
        let mut stmt = self.conn.prepare(sql)?;
        
        //結果を格納するVecです
        let mut result =Vec::new();

        //行数は実行時に決まる
        let column_count =stmt.column_count();

        //select文を実行
        let mut rows = stmt.query([])?;

        //1行ずつ取り出す
        while let Some(row)=rows.next()?
        {
            //1行分のデータ
            let mut current_row = Vec::new();

            //列数分ループする
            for i in 0..column_count{
                let value:Value = row.get(i)?;

                //ここで[String]型に変換しています
                let text = match value {
                    Value::Null => "NULL".to_string(),
                    Value::Integer(v) => v.to_string(),
                    Value::Real(v) => v.to_string(),
                    Value::Text(v) => v,
                    Value::Blob(_) => "[BLOB]".to_string(),
                };

                current_row.push(text);
            }

            //完成した1行を保存
            result.push(current_row);
        }
        Ok(result)
    }

    //SQLのSELECT文を実行して、複数の文字列を全部 Vec<Value> にして返す関数
    //関数[query_value]は型を変換しません
    pub fn query_as_value(&self,sql: &str,) -> Result<Vec<Vec<Value>>>
    {
        //sql文をここでコンパイルしています
        let mut stmt = self.conn.prepare(sql)?;
        
        //結果を格納するVecです
        let mut result =Vec::new();

        //行数は実行時に決まる
        let column_count =stmt.column_count();

        //select文を実行
        let mut rows = stmt.query([])?;

        //
        while let Some(row) =rows.next()?  
        {
            let mut current_row: Vec<Value>= Vec::new();

            for i in 0..column_count{
                let value: Value= row.get(i)?;
                current_row.push(value);
            }
            result.push(current_row);
            
        }

        Ok(result)
    }
    
}
