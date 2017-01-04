use errors::RiakErr;
use private_traits::YokozunaIndexPrivate;
use protobuf::{Message, RepeatedField};
use rpb::riak_yokozuna::{RpbYokozunaIndex, RpbYokozunaIndexPutReq};
use rpb::riak_search::RpbSearchQueryReq;

pub struct YokozunaIndex(RpbYokozunaIndex);

impl YokozunaIndex {
    // constructs a new `YokozunaIndex`
    pub fn new<T: Into<Vec<u8>>>(name: T) -> YokozunaIndex {
        let mut rpb_yokozuna_index = RpbYokozunaIndex::new();
        rpb_yokozuna_index.set_name(name.into());
        YokozunaIndex(rpb_yokozuna_index)
    }

    // get the `name` of the yokozuna index
    pub fn get_name(&self) -> Vec<u8> {
        self.0.get_name().to_vec()
    }

    // set the `name` of the yokozuna index
    pub fn set_name<T: Into<Vec<u8>>>(&mut self, name: T) {
        self.0.set_name(name.into());
    }

    // get the `schema` for the yokozuna index
    pub fn get_schema(&self) -> Option<Vec<u8>> {
        if self.0.has_schema() {
            Some(self.0.get_schema().to_vec())
        } else {
            None
        }
    }

    // set the `schema` for the yokozuna index
    pub fn set_schema<T: Into<Vec<u8>>>(&mut self, schema: T) {
        self.0.set_schema(schema.into());
    }

    // get the `n_val` for the yokozuna index
    pub fn get_n_val(&self) -> Option<u32> {
        if self.0.has_n_val() {
            Some(self.0.get_n_val())
        } else {
            None
        }
    }

    // set the `n_val` for the yokozuna index
    pub fn set_n_val(&mut self, n_val: u32) {
        self.0.set_n_val(n_val);
    }
}

impl YokozunaIndexPrivate for YokozunaIndex {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr> {
        let mut req = RpbYokozunaIndexPutReq::new();
        req.set_index(self.0.clone());
        match req.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => return Err(RiakErr::ProtobufError(error)),
        }
    }

    fn new_from_rpb_yokozuna_index(rpb_yokozuna_index: RpbYokozunaIndex) -> YokozunaIndex {
        YokozunaIndex(rpb_yokozuna_index)
    }
}

pub struct SearchQuery(RpbSearchQueryReq);

impl SearchQuery {
    /// constructs a new `SearchQuery`
    pub fn new<T: Into<Vec<u8>>>(q: T, index: T) -> SearchQuery {
        let mut req = RpbSearchQueryReq::new();
        req.set_q(q.into());
        req.set_index(index.into());
        SearchQuery(req)
    }

    /// retrieves the `q` field, containing the query
    pub fn get_q(&self) -> Vec<u8> {
        self.0.get_q().to_vec()
    }

    /// sets the `q` field with a new query
    pub fn set_q<T: Into<Vec<u8>>>(&mut self, q: T) {
        self.0.set_q(q.into());
    }

    /// retrieves the `index` field
    pub fn get_index(&self) -> Vec<u8> {
        self.0.get_index().to_vec()
    }

    /// sets the `index` field
    pub fn set_index<T: Into<Vec<u8>>>(&mut self, index: T) {
        self.0.set_index(index.into());
    }

    /// retrieves the `rows` field
    pub fn get_rows(&self) -> Option<u32> {
        if self.0.has_rows() {
            Some(self.0.get_rows())
        } else {
            None
        }
    }

    /// sets the `rows` field
    pub fn set_rows(&mut self, rows: u32) {
        self.0.set_rows(rows);
    }

    /// get the `start` field
    pub fn get_start(&self) -> Option<u32> {
        if self.0.has_start() {
            Some(self.0.get_start())
        } else {
            None
        }
    }

    /// sets the `start` field
    pub fn set_start(&mut self, start: u32) {
        self.0.set_start(start);
    }

    /// get the `sort` field
    pub fn get_sort(&self) -> Option<Vec<u8>> {
        if self.0.has_sort() {
            Some(self.0.get_sort().to_vec())
        } else {
            None
        }
    }

    /// set the `sort` field
    pub fn set_sort<T: Into<Vec<u8>>>(&mut self, sort: T) {
        self.0.set_sort(sort.into());
    }

    /// get the `filter` field
    pub fn get_filter(&self) -> Option<Vec<u8>> {
        if self.0.has_filter() {
            Some(self.0.get_filter().to_vec())
        } else {
            None
        }
    }

    /// set the `filter` field
    pub fn set_filter<T: Into<Vec<u8>>>(&mut self, filter: T) {
        self.0.set_filter(filter.into());
    }

    /// get the `df` field
    pub fn get_df(&self) -> Option<Vec<u8>> {
        if self.0.has_df() {
            Some(self.0.get_df().to_vec())
        } else {
            None
        }
    }

    /// set the `df` field
    pub fn set_df<T: Into<Vec<u8>>>(&mut self, df: T) {
        self.0.set_df(df.into());
    }

    /// get the `op` field
    pub fn get_op(&self) -> Option<Vec<u8>> {
        if self.0.has_op() {
            Some(self.0.get_op().to_vec())
        } else {
            None
        }
    }

    /// set the `op` field
    pub fn set_op<T: Into<Vec<u8>>>(&mut self, op: T) {
        self.0.set_op(op.into());
    }

    /// get the `fl` field
    pub fn get_fl(&self) -> Option<Vec<Vec<u8>>> {
        if self.0.get_fl().len() > 0 {
            Some(self.0.get_fl().to_vec())
        } else {
            None
        }
    }

    /// set the `fl` field
    pub fn set_fl(&mut self, fl: Vec<Vec<u8>>) {
        self.0.set_fl(RepeatedField::from_vec(fl));
    }

    /// get the `presort` field
    pub fn get_presort(&self) -> Option<Vec<u8>> {
        if self.0.has_presort() {
            Some(self.0.get_presort().to_vec())
        } else {
            None
        }
    }

    /// set the `presort` field
    pub fn set_presort<T: Into<Vec<u8>>>(&mut self, presort: T) {
        self.0.set_presort(presort.into());
    }
}
