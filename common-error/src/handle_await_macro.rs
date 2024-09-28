
/// [handle_await] 添加.await的展开。
///
/// # Example
///
/// ```no_run
///
/// struct LOC00007;
/// fn main(){
///     common_error::handle_await!(result: Result::Ok(""), etype: LOC00007, "x".clone() ,"y".clone());
/// }
///
/// ```
///

#[macro_export]
macro_rules! handle_await {
    (result: $result:expr, panic:$xx:expr, etype: $etype:ty , $($arg:tt)*)=> {
       match $result.await {
           Ok(re) => {re}
           Err(err) => {
                let (_, msg) =<$etype>::throw($($arg)*,err.to_string().as_str());
                tracing::error!(msg);
                panic!($xx);
           }
       }
    };
    (result: $result:expr, panic:$xx:expr, etype: $etype:ty , $($arg:expr)*)=> {
       match $result.await {
           Ok(re) => {re}
           Err(err) => {
                let (_, msg) =<$etype>::throw($($arg)*,err.to_string().as_str());
                tracing::error!(msg);
                panic!($xx);
           }
       }
    };

    (result: $result:expr,  etype: $etype:ty , $($arg:tt)*)=> {
       match $result.await {
           Ok(re) => {re}
           Err(err) => {
                let (err, msg) =<$etype>::throw($($arg)*,err.to_string().as_str());
                tracing::error!(msg);
                return Err(err);
           }
       }
    };
        (result: $result:expr,  etype: $etype:ty , $($arg:expr)*)=> {
       match $result.await {
           Ok(re) => {re}
           Err(err) => {
                let (err, msg) =<$etype>::throw($($arg)*,err.to_string().as_str());
                tracing::error!(msg);
                return Err(err);
           }
       }
    };
}