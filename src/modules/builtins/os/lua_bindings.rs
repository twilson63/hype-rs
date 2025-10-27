use super::operations::*;
use mlua::{Lua, Result as LuaResult, Table, Value};

pub fn create_os_module(lua: &Lua) -> LuaResult<Table> {
    let os = lua.create_table()?;

    let platform_fn = lua.create_function(|_, ()| Ok(platform()))?;
    os.set("platform", platform_fn)?;

    let arch_fn = lua.create_function(|_, ()| Ok(arch()))?;
    os.set("arch", arch_fn)?;

    let hostname_fn = lua.create_function(|_, ()| match hostname() {
        Ok(name) => Ok(name),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("hostname", hostname_fn)?;

    let homedir_fn = lua.create_function(|_, ()| match homedir() {
        Ok(dir) => Ok(dir),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("homedir", homedir_fn)?;

    let tmpdir_fn = lua.create_function(|_, ()| match tmpdir() {
        Ok(dir) => Ok(dir),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("tmpdir", tmpdir_fn)?;

    let cpus_fn = lua.create_function(|lua, ()| match cpus() {
        Ok(cpu_list) => {
            let result = lua.create_table()?;
            for (i, cpu) in cpu_list.iter().enumerate() {
                let cpu_table = lua.create_table()?;
                cpu_table.set("model", cpu.model.clone())?;
                cpu_table.set("speed", cpu.speed)?;
                result.set(i + 1, cpu_table)?;
            }
            Ok(result)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("cpus", cpus_fn)?;

    let totalmem_fn = lua.create_function(|_, ()| match totalmem() {
        Ok(mem) => Ok(mem),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("totalmem", totalmem_fn)?;

    let freemem_fn = lua.create_function(|_, ()| match freemem() {
        Ok(mem) => Ok(mem),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("freemem", freemem_fn)?;

    let uptime_fn = lua.create_function(|_, ()| match uptime() {
        Ok(time) => Ok(time),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("uptime", uptime_fn)?;

    let loadavg_fn = lua.create_function(|lua, ()| match loadavg() {
        Ok((one, five, fifteen)) => {
            let result = lua.create_table()?;
            result.set(1, one)?;
            result.set(2, five)?;
            result.set(3, fifteen)?;
            Ok(result)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("loadavg", loadavg_fn)?;

    let network_interfaces_fn = lua.create_function(|lua, ()| match network_interfaces() {
        Ok(interfaces) => {
            let result = lua.create_table()?;
            for (i, iface) in interfaces.iter().enumerate() {
                let iface_table = lua.create_table()?;
                iface_table.set("name", iface.name.clone())?;
                iface_table.set("mac", iface.mac_address.clone())?;
                result.set(i + 1, iface_table)?;
            }
            Ok(result)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("networkInterfaces", network_interfaces_fn)?;

    let user_info_fn = lua.create_function(|lua, ()| match user_info() {
        Ok(info) => {
            let result = lua.create_table()?;
            result.set("username", info.username)?;
            result.set("homedir", info.homedir)?;

            if let Some(uid) = info.uid {
                result.set("uid", uid)?;
            } else {
                result.set("uid", Value::Nil)?;
            }

            if let Some(gid) = info.gid {
                result.set("gid", gid)?;
            } else {
                result.set("gid", Value::Nil)?;
            }

            if let Some(shell) = info.shell {
                result.set("shell", shell)?;
            } else {
                result.set("shell", Value::Nil)?;
            }

            Ok(result)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    os.set("userInfo", user_info_fn)?;

    os.set("EOL", eol())?;

    Ok(os)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_create_os_module() {
        let lua = Lua::new();
        let result = create_os_module(&lua);
        assert!(result.is_ok());

        let os = result.unwrap();
        assert!(os.contains_key("platform").unwrap());
        assert!(os.contains_key("arch").unwrap());
        assert!(os.contains_key("hostname").unwrap());
        assert!(os.contains_key("homedir").unwrap());
        assert!(os.contains_key("tmpdir").unwrap());
        assert!(os.contains_key("cpus").unwrap());
        assert!(os.contains_key("totalmem").unwrap());
        assert!(os.contains_key("freemem").unwrap());
        assert!(os.contains_key("uptime").unwrap());
        assert!(os.contains_key("loadavg").unwrap());
        assert!(os.contains_key("networkInterfaces").unwrap());
        assert!(os.contains_key("userInfo").unwrap());
        assert!(os.contains_key("EOL").unwrap());
    }

    #[test]
    fn test_os_platform() {
        let lua = Lua::new();
        let os = create_os_module(&lua).unwrap();
        lua.globals().set("os", os).unwrap();

        let result: String = lua.load("return os.platform()").eval().unwrap();

        assert!(
            ["linux", "macos", "windows", "freebsd", "openbsd", "unknown"]
                .contains(&result.as_str())
        );
    }

    #[test]
    fn test_os_arch() {
        let lua = Lua::new();
        let os = create_os_module(&lua).unwrap();
        lua.globals().set("os", os).unwrap();

        let result: String = lua.load("return os.arch()").eval().unwrap();

        assert!(["x86_64", "aarch64", "arm", "x86", "unknown"].contains(&result.as_str()));
    }

    #[test]
    fn test_os_hostname() {
        let lua = Lua::new();
        let os = create_os_module(&lua).unwrap();
        lua.globals().set("os", os).unwrap();

        let result: String = lua.load("return os.hostname()").eval().unwrap();

        assert!(!result.is_empty());
    }

    #[test]
    fn test_os_eol() {
        let lua = Lua::new();
        let os = create_os_module(&lua).unwrap();
        lua.globals().set("os", os).unwrap();

        let result: String = lua.load("return os.EOL").eval().unwrap();

        #[cfg(target_os = "windows")]
        assert_eq!(result, "\r\n");
        #[cfg(not(target_os = "windows"))]
        assert_eq!(result, "\n");
    }
}
