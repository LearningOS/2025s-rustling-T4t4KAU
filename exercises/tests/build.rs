use std::time::SystemTime;

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = SystemTime::now()
       .duration_since(SystemTime::UNIX_EPOCH)
       .unwrap()
       .as_secs();
    // 设置环境变量 TEST_FOO 为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}    