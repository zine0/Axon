use std::fs;
use std::process::{Command, Stdio};

pub struct ContainerSandbox {
    container_id: String,
    rootfs: String,
}

impl ContainerSandbox {
    pub fn new(container_id: &str, rootfs: &str) -> anyhow::Result<Self> {
        // Create minimal rootfs directory structure
        fs::create_dir_all(rootfs)?;

        // Create essential directories in the rootfs
        fs::create_dir_all(format!("{}/bin", rootfs))?;
        fs::create_dir_all(format!("{}/usr/bin", rootfs))?;
        fs::create_dir_all(format!("{}/lib", rootfs))?;
        fs::create_dir_all(format!("{}/lib64", rootfs))?;
        fs::create_dir_all(format!("{}/etc", rootfs))?;
        fs::create_dir_all(format!("{}/proc", rootfs))?;
        fs::create_dir_all(format!("{}/sys", rootfs))?;
        fs::create_dir_all(format!("{}/dev", rootfs))?;
        fs::create_dir_all(format!("{}/dev/pts", rootfs))?;
        fs::create_dir_all(format!("{}/dev/shm", rootfs))?;
        fs::create_dir_all(format!("{}/workspace", rootfs))?;

        Ok(Self {
            container_id: container_id.to_string(),
            rootfs: rootfs.to_string(),
        })
    }

    pub fn copy_file_in(&self, src: &str, dest: &str) -> anyhow::Result<()> {
        fs::copy(src, format!("{}/{}", self.rootfs, dest))?;
        Ok(())
    }

    pub fn copy_file_out(&self, src: &str, dest: &str) -> anyhow::Result<()> {
        fs::copy(format!("{}/{}", self.rootfs, src), dest)?;
        Ok(())
    }

    pub fn run_command(&self, command: &str, args: &[&str]) -> anyhow::Result<String> {
        // Create container configuration with host mounts
        self.create_container_config(command, args)?;

        // Use runc to run the command
        let output = Command::new("runc")
            .args(&["run", "--bundle", &self.rootfs, &self.container_id])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(anyhow::anyhow!(
                "Command failed: {}",
                String::from_utf8(output.stderr)?
            ))
        }
    }

    fn create_container_config(&self, command: &str, args: &[&str]) -> anyhow::Result<()> {
        let mut full_args = vec![command];
        full_args.extend_from_slice(args);

        let config = serde_json::json!({
            "ociVersion": "1.0.0",
            "process": {
                "terminal": false,
                "user": {"uid": 0, "gid": 0},
                "args": full_args,
                "env": [
                    "PATH=/bin:/usr/bin:/usr/local/bin",
                    "HOME=/root",
                    "TERM=xterm"
                ],
                "cwd": "/workspace",
                "capabilities": {
                    "bounding": [],
                    "effective": [],
                    "inheritable": [],
                    "permitted": [],
                    "ambient": []
                }
            },
            "root": {"path": &self.rootfs,"readonly":false},
            "hostname": "sandbox",
            "mounts": [
                {
                    "destination": "/proc",
                    "type": "proc",
                    "source": "proc"
                },
                {
                    "destination": "/dev",
                    "type": "tmpfs",
                    "source": "tmpfs",
                    "options": ["nosuid", "strictatime", "mode=755", "size=65536k"]
                },
                  {
                    "destination": "/bin",
                    "type": "bind",
                    "source": "/usr/bin",
                    "options": ["rbind","ro", "nosuid", "nodev"]
                },
                {
                    "destination": "/usr/bin",
                    "type": "bind",
                    "source": "/usr/bin",
                    "options": ["rbind","ro", "nosuid", "nodev"]
                },
                {
                    "destination": "/lib",
                    "type": "bind",
                    "source": "/lib",
                    "options": ["rbind","ro", "nosuid", "nodev"]
                },
                {
                    "destination": "/lib64",
                    "type": "bind",
                    "source": "/lib64",
                    "options": ["rbind","ro", "nosuid", "nodev"]
                },
                {
                    "destination": "/usr/lib",
                    "type": "bind",
                    "source": "/usr/lib",
                    "options": ["rbind","ro", "nosuid", "nodev"]
                },
                {
                    "destination": "/usr/lib64",
                    "type": "bind",
                    "source": "/usr/lib64",
                    "options": ["rbind","ro", "nosuid", "nodev"]
                },
                {
                    "destination": "/workspace",
                    "type": "tmpfs",
                    "source": "tmpfs",
                    "options": ["rw", "nosuid", "nodev", "size=1048576k"]
                }
            ],
            "linux": {
                "resources": {
                    "devices": [{"allow": false, "access": "rwm"}]
                },
                "namespaces": [
                    {"type": "pid"},
                    {"type": "network"},
                    {"type": "ipc"},
                    {"type": "uts"},
                    {"type": "mount"},
                    {"type":"user"},
                ],
                "uidMappings":[
                    {"containerID":0, "hostID":1000, "size":1}
                ],
                "gidMappings":[
                    {"containerID":0, "hostID":1000, "size":1}
                ]
            }
        });

        fs::write(format!("{}/config.json", self.rootfs), config.to_string())?;
        Ok(())
    }

    pub fn cleanup(&self) -> anyhow::Result<()> {
        // Delete the container
        let _output = Command::new("runc")
            .args(&["delete", &self.container_id])
            .output()?;

        Ok(())
    }

    /// Clean up rootfs directory
    pub fn cleanup_rootfs(&self) -> anyhow::Result<()> {
        if std::path::Path::new(&self.rootfs).exists() {
            fs::remove_dir_all(&self.rootfs)?;
        }
        Ok(())
    }
}
