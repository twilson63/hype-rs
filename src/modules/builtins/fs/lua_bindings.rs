use super::operations::*;
use mlua::{Lua, Table};

pub fn create_fs_module(lua: &Lua) -> mlua::Result<Table> {
    let fs_table = lua.create_table()?;

    register_read_file_sync(lua, &fs_table)?;
    register_write_file_sync(lua, &fs_table)?;
    register_exists_sync(lua, &fs_table)?;
    register_stat_sync(lua, &fs_table)?;
    register_readdir_sync(lua, &fs_table)?;
    register_unlink_sync(lua, &fs_table)?;
    register_mkdir_sync(lua, &fs_table)?;
    register_rmdir_sync(lua, &fs_table)?;

    Ok(fs_table)
}

fn register_read_file_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let read_fn = lua.create_function(|_, path: String| {
        let content = read_file_sync(&path).map_err(mlua::Error::external)?;
        Ok(content)
    })?;
    table.set("readFileSync", read_fn)?;
    Ok(())
}

fn register_write_file_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let write_fn = lua.create_function(|_, (path, data): (String, String)| {
        write_file_sync(&path, &data).map_err(mlua::Error::external)?;
        Ok(())
    })?;
    table.set("writeFileSync", write_fn)?;
    Ok(())
}

fn register_exists_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let exists_fn = lua.create_function(|_, path: String| Ok(exists_sync(&path)))?;
    table.set("existsSync", exists_fn)?;
    Ok(())
}

fn register_stat_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let stat_fn = lua.create_function(move |lua, path: String| {
        let stat = stat_sync(&path).map_err(mlua::Error::external)?;

        let table = lua.create_table()?;
        table.set("size", stat.size)?;
        table.set("isFile", stat.is_file)?;
        table.set("isDirectory", stat.is_directory)?;
        table.set("isSymlink", stat.is_symlink)?;
        table.set("mtime", stat.mtime)?;

        Ok(table)
    })?;
    table.set("statSync", stat_fn)?;
    Ok(())
}

fn register_readdir_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let readdir_fn = lua.create_function(move |lua, path: String| {
        let files = readdir_sync(&path).map_err(mlua::Error::external)?;

        let table = lua.create_table()?;
        for (i, name) in files.iter().enumerate() {
            table.set(i + 1, name.as_str())?;
        }

        Ok(table)
    })?;
    table.set("readdirSync", readdir_fn)?;
    Ok(())
}

fn register_unlink_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let unlink_fn = lua.create_function(|_, path: String| {
        unlink_sync(&path).map_err(mlua::Error::external)?;
        Ok(())
    })?;
    table.set("unlinkSync", unlink_fn)?;
    Ok(())
}

fn register_mkdir_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let mkdir_fn = lua.create_function(|_, path: String| {
        mkdir_sync(&path).map_err(mlua::Error::external)?;
        Ok(())
    })?;
    table.set("mkdirSync", mkdir_fn)?;
    Ok(())
}

fn register_rmdir_sync(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let rmdir_fn = lua.create_function(|_, path: String| {
        rmdir_sync(&path).map_err(mlua::Error::external)?;
        Ok(())
    })?;
    table.set("rmdirSync", rmdir_fn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fs_module() {
        let lua = Lua::new();
        let result = create_fs_module(&lua);

        assert!(result.is_ok());
        let fs_table = result.unwrap();
        assert!(fs_table.contains_key("readFileSync").unwrap());
        assert!(fs_table.contains_key("writeFileSync").unwrap());
        assert!(fs_table.contains_key("existsSync").unwrap());
        assert!(fs_table.contains_key("statSync").unwrap());
        assert!(fs_table.contains_key("readdirSync").unwrap());
        assert!(fs_table.contains_key("unlinkSync").unwrap());
        assert!(fs_table.contains_key("mkdirSync").unwrap());
        assert!(fs_table.contains_key("rmdirSync").unwrap());
    }
}
