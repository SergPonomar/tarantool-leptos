local fio = require('fio')

local base_dir = os.environ()["TARANTOOL_RUNNER_BASEDIR"]
local tmpdir = os.environ()["TARANTOOL_RUNNER_TMPDIR"]
local listen_socket =  base_dir .. "/tnt.sock"

local package_location = os.environ()["TARANTOOL_RUNNER_PACKAGE_LOCATION"]
local package_name = os.environ()["TARANTOOL_RUNNER_PACKAGE_NAME"]
local package_entrypoint = package_name .. "." .. os.environ()["TARANTOOL_RUNNER_PACKAGE_ENTRYPOINT"]

local input = os.environ()["TARANTOOL_RUNNER_INPUT"]

box.cfg {
    listen = listen_socket,
    wal_mode = 'none',
    memtx_dir = tmpdir,
    wal_dir = tmpdir,
    election_mode = 'manual',
    log_level = 5,
    memtx_max_tuple_size = 8388608
}

box.ctl.promote()

fio.rmtree(tmpdir)

-- Add executable location to lua search path
package.cpath = package_location .. '/?.so;' .. package_location .. '/?.dylib;' .. package.cpath

-- Run tests
box.schema.func.create(package_entrypoint, { language = 'C', if_not_exists = true })

box.func[package_entrypoint]:call({input})

