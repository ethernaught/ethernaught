use core::ptr;
use core::ffi::c_void;
use core::ffi::CStr;
use std::ffi::CString;

pub struct Database {
    db: *mut c_void
}

impl Database {

    pub fn open(name: &str) -> Option<Self> {
        let c_db_name = CString::new(name).unwrap();
        let mut db: *mut c_void = ptr::null_mut();
        let rc = unsafe { sqlite3_open(c_db_name.as_ptr(), &mut db) };
        if rc != 0 {
            return None;
        }

        let create_table = "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        );";
        execute_sql(db, create_table);

        Some(Self {
            db
        })
    }

    pub fn insert(&mut self, name: &str, age: i32) {
        let sql = format!("INSERT INTO users (name, age) VALUES ('{}', {});", name, age);
        execute_sql(self.db, &sql);
    }

    pub fn get(&self) {
        let query = CString::new("SELECT id, name, age FROM users;").unwrap();
        unsafe {
            sqlite3_exec(
                self.db,
                query.as_ptr(),
                Some(query_callback),
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }

    pub fn close(&mut self) {
        unsafe { sqlite3_close(self.db) };
    }
}

fn execute_sql(db: *mut c_void, sql: &str) {
    let c_sql = CString::new(sql).unwrap();
    unsafe { sqlite3_exec(db, c_sql.as_ptr(), None, ptr::null_mut(), ptr::null_mut()) };
}

#[link(name = "sqlite3")]
extern "C" {
    fn sqlite3_open(filename: *const i8, db: *mut *mut c_void) -> i32;

    fn sqlite3_exec(
        db: *mut c_void,
        sql: *const i8,
        callback: Option<extern "C" fn(*mut c_void, i32, *mut *mut i8, *mut *mut i8) -> i32>,
        arg: *mut c_void,
        errmsg: *mut *mut i8,
    ) -> i32;

    fn sqlite3_close(db: *mut c_void) -> i32;
}

extern "C" fn query_callback(_arg: *mut c_void, column_count: i32, column_values: *mut *mut i8, _column_names: *mut *mut i8) -> i32 {
    for i in 0..column_count {
        let value = unsafe { CStr::from_ptr(*column_values.offset(i as isize)) };
        print!("{} ", value.to_string_lossy());
    }
    println!();
    0
}
