//ログレベル（Log Level）とは、ログの重要度や目的を分類するための段階です。
//プログラムの実行中には様々な情報を出力したくなりますが、すべて同じ扱いにすると必要な情報が見つけにくくなります。そこで、重要度ごとにレベルを分けます。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,  //開発用
    Info,   //通常情報
    Warn,   //警告
    Error,  //エラー
}