use std::marker::PhantomData;
use std::collections::HashMap;

pub struct Database {
    data: Vec<u8>,
}

pub struct Transaction<'a> {
    db: &'a Database,
    active: bool,
}

pub struct Cursor<'a> {
    txn: &'a Transaction<'a>,
    pos: usize,
}

pub struct Row<'a> {
    data: &'a [u8],
    txn: &'a Transaction<'a>,
}

pub struct Field<'a> {
    bytes: &'a [u8],
}

impl Database {
    pub fn new() -> Self {
        Self { data: vec![0; 1024] }
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction {
            db: self,
            active: true,
        }
    }
}

impl<'a> Transaction<'a> {
    pub fn cursor(&'a self) -> Cursor<'a> {
        Cursor {
            txn: self,
            pos: 0,
        }
    }
    
    pub fn get_blob(&'a self, offset: usize, len: usize) -> Option<&'a [u8]> {
        if offset + len <= self.db.data.len() {
            Some(&self.db.data[offset..offset+len])
        } else {
            None
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn next(&mut self) -> Option<Row<'a>> {
        if self.pos + 10 <= self.txn.db.data.len() {
            let row_data = &self.txn.db.data[self.pos..self.pos+10];
            self.pos += 10;
            Some(Row {
                data: row_data,
                txn: self.txn,
            })
        } else {
            None
        }
    }
}

impl<'a> Row<'a> {
    pub fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        if idx < 5 { 
            let start = idx * 2;
             Some(Field {
                 bytes: &self.data[start..start+2]
             })
        } else {
            None
        }
    }
    
    pub fn get_raw(&self) -> &'a [u8] {
        self.data
    }
}

pub struct PreparedStatement<'a> {
    txn: &'a Transaction<'a>,
    query: &'a str,
}

impl<'a> PreparedStatement<'a> {
    pub fn prepare(txn: &'a Transaction<'a>, query: &'a str) -> Self {
        Self { txn, query }
    }
    
    pub fn execute(&self) -> Cursor<'a> {
        self.txn.cursor()
    }
}

pub struct ComplexQueryBuilder<'a> {
    txn: &'a Transaction<'a>,
    parts: Vec<&'a str>,
}

impl<'a> ComplexQueryBuilder<'a> {
    pub fn new(txn: &'a Transaction<'a>) -> Self {
        Self { txn, parts: Vec::new() }
    }
    
    pub fn add_clause(&mut self, clause: &'a str) {
        self.parts.push(clause);
    }
    
    pub fn build(self) -> PreparedStatement<'a> {
        let _combined = self.parts.join(" "); 
        PreparedStatement {
            txn: self.txn,
            query: "dummy", 
        }
    }
}

pub struct ResultIterator<'a, 'b> 
where 'a: 'b {
    cursor: &'b mut Cursor<'a>,
}

impl<'a, 'b> Iterator for ResultIterator<'a, 'b> {
    type Item = Row<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next()
    }
}

pub struct RefCellLike<'a, T> {
    value: T,
    borrow_state: usize,
    _marker: PhantomData<&'a T>,
}

pub struct Ref<'a, T> {
    value: &'a T,
}

impl<'a, T> RefCellLike<'a, T> {
    pub fn borrow(&'a self) -> Ref<'a, T> {
        Ref { value: &self.value }
    }
}

pub struct ConnectionPool<'a> {
    dbs: Vec<&'a Database>,
}

impl<'a> ConnectionPool<'a> {
    pub fn get_conn(&self) -> Option<&'a Database> {
        self.dbs.first().copied()
    }
}

pub struct ScopedTransaction<'a, F> 
where F: FnOnce(&Transaction<'a>)
{
    db: &'a Database,
    func: F,
}

impl<'a, F> ScopedTransaction<'a, F> 
where F: FnOnce(&Transaction<'a>)
{
    pub fn run(self) {
        let txn = self.db.begin_transaction();
        (self.func)(&txn);
    }
}

pub struct View<'a> {
    rows: Vec<Row<'a>>,
}

impl<'a> View<'a> {
    pub fn from_cursor(mut cursor: Cursor<'a>) -> Self {
        let mut rows = Vec::new();
        while let Some(row) = cursor.next() {
            rows.push(row);
        }
        Self { rows }
    }
    
    pub fn filter<P>(&'a self, predicate: P) -> View<'a>
    where P: Fn(&Row<'a>) -> bool 
    {
        let mut filtered = Vec::new();
        for r in &self.rows {
            if predicate(r) {
                filtered.push(Row { data: r.data, txn: r.txn });
            }
        }
        View { rows: filtered }
    }
}

pub trait Decoder<'a> {
    fn decode(field: &Field<'a>) -> Self;
}

impl<'a> Decoder<'a> for u16 {
    fn decode(field: &Field<'a>) -> Self {
        if field.bytes.len() >= 2 {
            u16::from_le_bytes([field.bytes[0], field.bytes[1]])
        } else {
            0
        }
    }
}

pub struct Record<'a> {
    id: u16,
    name: &'a str,
}

pub struct RecordBatch<'a> {
    records: Vec<Record<'a>>,
}

pub struct BatchIterator<'a> {
    batch: &'a RecordBatch<'a>,
    idx: usize,
}

impl<'a> Iterator for BatchIterator<'a> {
    type Item = &'a Record<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.batch.records.len() {
            let r = &self.batch.records[self.idx];
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }
}

pub struct JoinedCursor<'a, 'b> {
    left: Cursor<'a>,
    right: Cursor<'b>,
}

impl<'a, 'b> JoinedCursor<'a, 'b> {
    pub fn next(&mut self) -> Option<(Row<'a>, Row<'b>)> {
        match (self.left.next(), self.right.next()) {
            (Some(l), Some(r)) => Some((l, r)),
            _ => None,
        }
    }
}

pub struct AsyncLikeCursor<'a> {
    cursor: Cursor<'a>,
}

impl<'a> AsyncLikeCursor<'a> {
    pub fn poll(&mut self, _cx: &mut ()) -> Option<Row<'a>> {
        self.cursor.next()
    }
}

pub struct TripleLifetime<'a, 'b, 'c, T> {
    x: &'a T,
    y: &'b T,
    z: &'c T,
}

pub fn select_longest_lifetime<'a, 'b, 'c, T>(
    t: TripleLifetime<'a, 'b, 'c, T>
) -> &'a T 
where 'b: 'a, 'c: 'a 
{
    t.y 
}

fn main() {
    let db = Database::new();
    let txn = db.begin_transaction();
    let mut cursor = txn.cursor();
    while let Some(_) = cursor.next() {}
}
