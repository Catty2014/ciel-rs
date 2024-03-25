//! # DBus interface proxy for: `org.freedesktop.machine1.Manager`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `org.freedesktop.machine1.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.machine1.Manager",
    default_service = "org.freedesktop.machine1",
    default_path = "/org/freedesktop/machine1"
)]
trait Manager {
    /// BindMountMachine method
    fn bind_mount_machine(
        &self,
        name: &str,
        source: &str,
        destination: &str,
        read_only: bool,
        mkdir: bool,
    ) -> zbus::Result<()>;

    /// CleanPool method
    fn clean_pool(&self, mode: &str) -> zbus::Result<Vec<(String, u64)>>;

    /// CloneImage method
    fn clone_image(&self, name: &str, new_name: &str, read_only: bool) -> zbus::Result<()>;

    /// CopyFromMachine method
    fn copy_from_machine(&self, name: &str, source: &str, destination: &str) -> zbus::Result<()>;

    /// CopyToMachine method
    fn copy_to_machine(&self, name: &str, source: &str, destination: &str) -> zbus::Result<()>;

    /// CreateMachine method
    fn create_machine(
        &self,
        name: &str,
        id: &[u8],
        service: &str,
        class: &str,
        leader: u32,
        root_directory: &str,
        scope_properties: &[(&str, zbus::zvariant::Value<'_>)],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// CreateMachineWithNetwork method
    fn create_machine_with_network(
        &self,
        name: &str,
        id: &[u8],
        service: &str,
        class: &str,
        leader: u32,
        root_directory: &str,
        ifindices: &[i32],
        scope_properties: &[(&str, zbus::zvariant::Value<'_>)],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetImage method
    fn get_image(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetImageHostname method
    fn get_image_hostname(&self, name: &str) -> zbus::Result<String>;

    /// GetImageMachineID method
    fn get_image_machine_id(&self, name: &str) -> zbus::Result<Vec<u8>>;

    /// GetImageMachineInfo method
    fn get_image_machine_info(
        &self,
        name: &str,
    ) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// GetImageOSRelease method
    fn get_image_osrelease(
        &self,
        name: &str,
    ) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// GetMachine method
    fn get_machine(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetMachineAddresses method
    fn get_machine_addresses(&self, name: &str) -> zbus::Result<Vec<(i32, Vec<u8>)>>;

    /// GetMachineByPID method
    fn get_machine_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetMachineOSRelease method
    fn get_machine_osrelease(
        &self,
        name: &str,
    ) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// GetMachineUIDShift method
    fn get_machine_uidshift(&self, name: &str) -> zbus::Result<u32>;

    /// KillMachine method
    fn kill_machine(&self, name: &str, who: &str, signal: i32) -> zbus::Result<()>;

    /// ListImages method
    fn list_images(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            bool,
            u64,
            u64,
            u64,
            zbus::zvariant::OwnedObjectPath,
        )>,
    >;

    /// ListMachines method
    fn list_machines(
        &self,
    ) -> zbus::Result<Vec<(String, String, String, zbus::zvariant::OwnedObjectPath)>>;

    /// MapFromMachineGroup method
    fn map_from_machine_group(&self, name: &str, gid_inner: u32) -> zbus::Result<u32>;

    /// MapFromMachineUser method
    fn map_from_machine_user(&self, name: &str, uid_inner: u32) -> zbus::Result<u32>;

    /// MapToMachineGroup method
    fn map_to_machine_group(
        &self,
        gid_outer: u32,
    ) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath, u32)>;

    /// MapToMachineUser method
    fn map_to_machine_user(
        &self,
        uid_outer: u32,
    ) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath, u32)>;

    /// MarkImageReadOnly method
    fn mark_image_read_only(&self, name: &str, read_only: bool) -> zbus::Result<()>;

    /// OpenMachineLogin method
    fn open_machine_login(&self, name: &str) -> zbus::Result<(zbus::zvariant::OwnedFd, String)>;

    /// OpenMachinePTY method
    fn open_machine_pty(&self, name: &str) -> zbus::Result<(zbus::zvariant::OwnedFd, String)>;

    /// OpenMachineRootDirectory method
    fn open_machine_root_directory(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// OpenMachineShell method
    fn open_machine_shell(
        &self,
        name: &str,
        user: &str,
        path: &str,
        args: &[&str],
        environment: &[&str],
    ) -> zbus::Result<(zbus::zvariant::OwnedFd, String)>;

    /// RegisterMachine method
    fn register_machine(
        &self,
        name: &str,
        id: &[u8],
        service: &str,
        class: &str,
        leader: u32,
        root_directory: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// RegisterMachineWithNetwork method
    fn register_machine_with_network(
        &self,
        name: &str,
        id: &[u8],
        service: &str,
        class: &str,
        leader: u32,
        root_directory: &str,
        ifindices: &[i32],
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// RemoveImage method
    fn remove_image(&self, name: &str) -> zbus::Result<()>;

    /// RenameImage method
    fn rename_image(&self, name: &str, new_name: &str) -> zbus::Result<()>;

    /// SetImageLimit method
    fn set_image_limit(&self, name: &str, size: u64) -> zbus::Result<()>;

    /// SetPoolLimit method
    fn set_pool_limit(&self, size: u64) -> zbus::Result<()>;

    /// TerminateMachine method
    fn terminate_machine(&self, id: &str) -> zbus::Result<()>;

    /// UnregisterMachine method
    fn unregister_machine(&self, name: &str) -> zbus::Result<()>;

    /// MachineNew signal
    #[zbus(signal)]
    fn machine_new(&self, machine: &str, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// MachineRemoved signal
    #[zbus(signal)]
    fn machine_removed(
        &self,
        machine: &str,
        path: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// PoolLimit property
    #[zbus(property)]
    fn pool_limit(&self) -> zbus::Result<u64>;

    /// PoolPath property
    #[zbus(property)]
    fn pool_path(&self) -> zbus::Result<String>;

    /// PoolUsage property
    #[zbus(property)]
    fn pool_usage(&self) -> zbus::Result<u64>;
}
