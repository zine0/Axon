use std::fs;

use sandbox::ContainerSandbox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rootfs_path = "/tmp/sandbox_rootfs";
    let container_id = "test_sandbox";

    // 清理之前的测试
    let _ = fs::remove_dir_all(rootfs_path);

    // 使用 busybox 创建 rootfs
    let sandbox = ContainerSandbox::new(container_id, rootfs_path)?;

    println!("Sandbox created with busybox at: {}", rootfs_path);

    // 测试基本命令
    match sandbox.run_command("/bin/ls", &["-la", "/"]) {
        Ok(output) => println!("Command output:\n{}", output),
        Err(e) => println!("Error running command: {}", e),
    }

    // 测试 uname
    match sandbox.run_command("/bin/uname", &["-a"]) {
        Ok(output) => println!("System info:\n{}", output),
        Err(e) => println!("Error running uname: {}", e),
    }

    sandbox.cleanup()?;
    sandbox.cleanup_rootfs()?;

    Ok(())
}
