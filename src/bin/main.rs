#[macro_use]

extern crate cpython;
use cpython::{Python, PyDict, PyList, PyIterator, PyResult};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use phf::phf_map;

#[derive(Debug)]
struct FieldType {
    name: &'static str,
    typename: &'static str,
}

static FIELD_MAPPINGS: phf::Map<&'static str, FieldType> = phf_map! {
    "a" => FieldType {
        name: "access_mode",
        typename: "str"
    },
    "c" => FieldType {name: "command", typename: "str"},
    "C" => FieldType {name: "structure_share_count", typename: "int"},
    "d" => FieldType {name: "device_character_code", typename: "str"},
    "D" => FieldType {name: "device_number", typename: "str"},
    "f" => FieldType {name: "descriptor", typename: "str"},
    "F" => FieldType {name: "structure_address", typename: "str"},
    "G" => FieldType {name: "flags", typename: "str"},
    "g" => FieldType {name: "gid", typename: "int"},
    "i" => FieldType {name: "inode_number", typename: "int"},
    "K" => FieldType {name: "task_id", typename: "int"},
    "k" => FieldType {name: "link_count", typename: "int"},
    "l" => FieldType {name: "lock_status", typename: "str"},
    "L" => FieldType {name: "login_name", typename: "str"},
    "m" => FieldType {name: "repeated_output_marker", typename: "str"},
    "M" => FieldType {name: "task_command", typename: "str"},
    "n" => FieldType {name: "file_name", typename: "str"},
    "N" => FieldType {name: "node_identifier", typename: "str"},
    "o" => FieldType {name: "offset", typename: "str"},
    "p" => FieldType {name: "pid", typename: "int"},
    "P" => FieldType {name: "protocol_name", typename: "str"},
    "r" => FieldType {name: "raw_device_number", typename: "str"},
    "R" => FieldType {name: "ppid", typename: "int"},
    "s" => FieldType {name: "size", typename: "int"},
    "S" => FieldType {name: "stream", typename: "str"},
    "t" => FieldType {name: "type", typename: "str"},
    "TQR" => FieldType {name: "tcp_read_queue_size", typename: "int"},
    "TQS" => FieldType {name: "tcp_send_queue_size", typename: "int"},
    "TSO" => FieldType {name: "tcp_socket_options", typename: "str"},
    "TSS" => FieldType {name: "tcp_socket_states", typename: "str"},
    "TST" => FieldType {name: "tcp_connection_state", typename: "str"},
    "TTF" => FieldType {name: "tcp_flags", typename: "str"},
    "TWR" => FieldType {name: "tcp_window_read_size", typename: "int"},
    "TWS" => FieldType {name: "tcp_window_write_size", typename: "int"},
    "u" => FieldType {name: "uid", typename: "int"},
    "z" => FieldType {name: "zone_name", typename: "str"},
    "Z" => FieldType {name: "selinux_security_context", typename: "str"},
    "0" => FieldType {name: "use_nul_sep", typename: "str"},
    "1" => FieldType {name: "dialect_specific_1", typename: "str"},
    "2" => FieldType {name: "dialect_specific_2", typename: "str"},
    "3" => FieldType {name: "dialect_specific_3", typename: "str"},
    "4" => FieldType {name: "dialect_specific_4", typename: "str"},
    "5" => FieldType {name: "dialect_specific_5", typename: "str"},
    "6" => FieldType {name: "dialect_specific_6", typename: "str"},
    "7" => FieldType {name: "dialect_specific_7", typename: "str"},
    "8" => FieldType {name: "dialect_specific_8", typename: "str"},
    "9" => FieldType {name: "dialect_specific_9", typename: "str"},
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_kvn(rec: String) -> (String, String, bool) {
    if rec.starts_with("T") {
        let parts: Vec<_> = rec.splitn(2, "=").collect();
        (parts[0].to_string(), parts[1].to_string(), true)
    } else {
        let (key, value) = rec.split_at(1);
        (key.to_string(), value.to_string(), false)
    }
}

fn parse_lines<'a, I>(lines: I) -> Vec<HashMap<&'a str, String>>
    where
    I: Iterator<Item = Result<String, std::io::Error>>
{
    let mut one: HashMap<&str, String> = HashMap::new();
    let mut all: Vec<_> = Vec::new();

    for line in lines {
        if let Ok(rec) = line {
            let (key, value, _net) = get_kvn(rec);
            if let Some(f) = FIELD_MAPPINGS.get(key.as_str()) {
               if one.contains_key(f.name) {
                   all.push(one);
                   one = HashMap::new();
                   continue;
               }
               one.insert(f.name, value);
            }
        }
    }
    all
}

fn parse<I>(py: Python, it: I) -> PyResult<PyList> 
    where
    I: Iterator<Item = Result<String, std::io::Error>>
{
    
}

fn main() {
    if let Ok(lines) = read_lines("lsof.output") {
        let parsed = parse_lines(lines);
        println!("{:?}", parsed);
    }
} 

py_module_initializer!(rust2py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "parse", py_fn!(py, parse(it:PyIterator)))?;
    Ok(())
});
