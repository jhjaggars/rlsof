extern crate phf;
extern crate pyo3;

use phf::phf_map;
use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub enum Val {
    Str(String),
    Num(String),
}

impl pyo3::conversion::IntoPy<PyObject> for Val {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Val::Str(v) => pyo3::types::PyString::new(py, &v).into(),
            Val::Num(v) => {
                if let Ok(n) = v.parse::<i64>() {
                    n.to_object(py)
                } else {
                    pyo3::types::PyString::new(py, &v).into()
                }
            }
        }
    }
}

#[derive(Debug)]
struct FieldType {
    name: &'static str,
    typename: &'static str,
}

impl FieldType {
    fn val(&self, s: String) -> Result<Val, &'static str>{
        match self.typename {
            "str" => Ok(Val::Str(s)),
            "int" => Ok(Val::Num(s)),
            _ => Err("invalid field type"),
        }
    }
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

fn get_kvn(rec: &str) -> (&str, &str, bool) {
    if rec.starts_with("T") {
        let parts: Vec<_> = rec.splitn(2, "=").collect();
        (parts[0], parts[1], true)
    } else {
        let (key, value) = rec.split_at(1);
        (key, value, false)
    }
}

fn map_record(field: &str, record: &mut HashMap<&str, Val>) {
    let (k, v, _n) = get_kvn(field);
    if let Some(f) = FIELD_MAPPINGS.get(k) {
        if let Ok(value) = f.val(v.to_string()) {
            record.insert(f.name, value);
        }
    }
}

pub fn map_line(line: &str, mut record: &mut HashMap<&str, Val>) {
    let sp = line.trim_end_matches('\0').split('\0');
    sp.for_each(|field| {
        map_record(field, &mut record);
    });
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_from_file(filename: &str) -> Result<Vec<HashMap<&str, Val>>, std::io::Error> {
    match read_lines(filename) {
        Ok(lines) => {
            let res: Vec<_> = lines.filter_map(|line| {
                let mut rec: HashMap<&str, Val> = HashMap::new();
                map_line(&line.ok()?, &mut rec);
                Some(rec)
            }).collect();
            Ok(res)
        },
        Err(e) => Err(e)
    }
}

#[pyfunction]
fn parse(filename: &str) -> PyResult<Vec<HashMap<&str, Val>>> {
    match parse_from_file(filename) {
        Ok(r) => Ok(r),
        Err(e) => Err(exceptions::IOError::py_err(e.to_string()))
    }
}

#[pymodule]
fn rlsof(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse))?;
    Ok(())
}

#[test]
fn parse_kvn() {
    let inp = "a50a701b1dcb4d7d92bae1fcd64ee065";
    let (k, v, n) = get_kvn(inp);
    assert_eq!(k, "a");
    assert_eq!(v, "50a701b1dcb4d7d92bae1fcd64ee065");
    assert_eq!(n, false);
}

#[test]
fn parse_kvn_short() {
    let inp = "a ";
    let (k, v, n) = get_kvn(inp);
    assert_eq!(k, "a");
    assert_eq!(v, " ");
    assert_eq!(n, false);
}

#[test]
fn field_maps() {
    let inp = "a50a701b1dcb4d7d92bae1fcd64ee065";
    let (k, _, _) = get_kvn(inp);
    let fm = FIELD_MAPPINGS.get(k).unwrap();
    assert_eq!(fm.name, "access_mode")
}

#[test]
fn maps_full_record() {
    let inp = "p195\0g195\0R1\0cloginwindow\0u501\0Ljjaggars\0";
    let mut hm: HashMap<&str, String> = HashMap::new();
    map_line(inp, &mut hm);
    assert_eq!(hm.get("login_name"), Some(&String::from("jjaggars")));
}

#[test]
fn maps_multiple_lines_file() {
    let inp = "p195 g195 R1 cloginwindow u501 Ljjaggars \n\
               fcwd a  l  tDIR D0x1000004 s704 i2 k22 n/ ";
    let hm: HashMap<&str, String> = inp.lines().map(|line| {
        let mut record: HashMap<&str, String> = HashMap::new();
        map_line(line, &mut record);
        record
    }).last().unwrap();
    assert_eq!(hm.get("descriptor"), Some(&String::from("cwd")));
}


