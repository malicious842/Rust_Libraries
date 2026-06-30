use log_service::{Logger, LoggerConfig, level};

fn main()
{
    //ログレベルの比較テストをしています
    println!("{:?}",log_service::LogLevel::Debug);
    println!("{:?}",log_service::LogLevel::Debug < log_service::LogLevel::Info);
    println!("{:?}",log_service::LogLevel::Debug > log_service::LogLevel::Warn);

    //ログファイル作成テスト
    let logger= Logger::new(
        LoggerConfig {
             level: level::LogLevel::Debug,
            console:true,
            file:Some("test.log".to_string()),});
    logger.log(level::LogLevel::Info, "テストログです");

    println!("出力完了")

}