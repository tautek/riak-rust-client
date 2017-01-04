/// Object related structs and functions for Riak objects.
///
/// For more information: https://docs.basho.com/riak/kv/latest/developing/usage/creating-objects/

use errors::RiakErr;
use private_traits::DeleteObjectReqPrivate;
use protobuf::Message;
use rpb::riak_kv::RpbDelReq;
use rpb::utils::{fetch_object_req_to_rpb_get_req, store_object_req_to_rpb_put_req, RpbGenerator};

/// `DeleteObjectReq` represents a request to delete an object from Riak
#[derive(Clone, Debug)]
pub struct DeleteObjectReq(RpbDelReq);

impl DeleteObjectReq {
    /// constructs a new `DeleteObjectReq`
    pub fn new<T: Into<Vec<u8>>>(bucket: T, key: T) -> DeleteObjectReq {
        let mut req = DeleteObjectReq(RpbDelReq::new());
        req.0.set_bucket(bucket.into());
        req.0.set_key(key.into());
        req
    }

    /// get the value of the "bucket_type" property
    pub fn get_bucket_type(&self) -> Option<Vec<u8>> {
        if self.0.has_field_type() {
            Some(self.0.get_field_type().to_vec())
        } else {
            None
        }
    }

    /// set the value of the "bucket_type" property
    pub fn set_bucket_type<T: Into<Vec<u8>>>(&mut self, bucket_type: T) {
        self.0.set_field_type(bucket_type.into());
    }

    /// get the value of the "bucket" property
    pub fn get_bucket(&self) -> Vec<u8> {
        self.0.get_bucket().to_vec()
    }

    /// set the value of the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.0.set_bucket(bucket.into());
    }

    /// get the value of the "key" property
    pub fn get_key(&self) -> Vec<u8> {
        self.0.get_key().to_vec()
    }

    /// set the value of the "key" property
    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.0.set_key(key.into());
    }

    /// get the value of the "dw" property
    pub fn get_dw(&self) -> Option<u32> {
        if self.0.has_dw() {
            Some(self.0.get_dw())
        } else {
            None
        }
    }

    /// set the value of the "dw" property
    pub fn set_dw(&mut self, dw: u32) {
        self.0.set_dw(dw);
    }

    /// get the value of the "pr" property
    pub fn get_pr(&self) -> Option<u32> {
        if self.0.has_pr() {
            Some(self.0.get_pr())
        } else {
            None
        }
    }

    /// set the value of the "pr" property
    pub fn set_pr(&mut self, pr: u32) {
        self.0.set_pr(pr);
    }

    /// get the value of the "pw" property
    pub fn get_pw(&self) -> Option<u32> {
        if self.0.has_pw() {
            Some(self.0.get_pw())
        } else {
            None
        }
    }

    /// set the value of the "pw" property
    pub fn set_pw(&mut self, pw: u32) {
        self.0.set_pw(pw);
    }

    /// get the value of the "rw" property
    pub fn get_rw(&self) -> Option<u32> {
        if self.0.has_rw() {
            Some(self.0.get_rw())
        } else {
            None
        }
    }

    /// set the value of the "rw" property
    pub fn set_rw(&mut self, rw: u32) {
        self.0.set_rw(rw);
    }

    /// get the value of the "r" property
    pub fn get_r(&self) -> Option<u32> {
        if self.0.has_r() {
            Some(self.0.get_r())
        } else {
            None
        }
    }

    /// set the value of the "r" property
    pub fn set_r(&mut self, r: u32) {
        self.0.set_r(r);
    }

    /// get the value of the "w" property
    pub fn get_w(&self) -> Option<u32> {
        if self.0.has_w() {
            Some(self.0.get_w())
        } else {
            None
        }
    }

    /// set the value of the "w" property
    pub fn set_w(&mut self, w: u32) {
        self.0.set_w(w);
    }

    /// get the value of the "n_val" property
    pub fn get_n_val(&self) -> Option<u32> {
        if self.0.has_n_val() {
            Some(self.0.get_n_val())
        } else {
            None
        }
    }

    /// set the value of the "n_val" property
    pub fn set_n_val(&mut self, n_val: u32) {
        self.0.set_n_val(n_val);
    }

    /// get the value of the "sloppy_quorum" property
    pub fn get_sloppy_quorum(&self) -> Option<bool> {
        if self.0.has_sloppy_quorum() {
            Some(self.0.get_sloppy_quorum())
        } else {
            None
        }
    }

    /// set the value of the "sloppy_quorum" property
    pub fn set_sloppy_quorum(&mut self, sloppy_quorum: bool) {
        self.0.set_sloppy_quorum(sloppy_quorum);
    }

    /// get the value of the "timeout" property
    pub fn get_timeout(&self) -> Option<u32> {
        if self.0.has_timeout() {
            Some(self.0.get_timeout())
        } else {
            None
        }
    }

    /// set the value of the "timeout" property
    pub fn set_timeout(&mut self, timeout: u32) {
        self.0.set_timeout(timeout);
    }

    /// get the value of the "vclock" property
    pub fn get_vclock(&self) -> Option<Vec<u8>> {
        if self.0.has_vclock() {
            let vclock = self.0.get_vclock().to_vec();
            Some(vclock)
        } else {
            None
        }
    }

    /// set the value of the "vclock" property
    pub fn set_vclock<T: Into<Vec<u8>>>(&mut self, vclock: T) {
        self.0.set_vclock(vclock.into());
    }
}

impl DeleteObjectReqPrivate for DeleteObjectReq {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr> {
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(RiakErr::ProtobufError(error)),
        }
    }
}

/// The data used to perform an store object request.
///
/// # Examples
///
/// ```
/// ```
#[derive(Clone, Debug)]
pub struct StoreObjectReq {
    // Required
    bucket: Vec<u8>,
    content: ObjectContent,
    // optional
    asis: Option<bool>,
    bucket_type: Option<Vec<u8>>,
    if_none_match: Option<bool>,
    if_not_modified: Option<bool>,
    key: Option<Vec<u8>>,
    n_val: Option<u32>,
    return_body: Option<bool>,
    return_head: Option<bool>,
    sloppy_quorum: Option<bool>,
    timeout: Option<u32>,
    vclock: Option<Vec<u8>>,
    // optional w
    dw: Option<u32>,
    pw: Option<u32>,
    w: Option<u32>,
}

impl StoreObjectReq {
    pub fn new<T: Into<Vec<u8>>>(bucket: T, content: ObjectContent) -> StoreObjectReq {
        StoreObjectReq {
            // required
            bucket: bucket.into(),
            content: content,
            // optional
            asis: None,
            bucket_type: None,
            if_none_match: None,
            if_not_modified: None,
            key: None,
            n_val: None,
            return_body: None,
            return_head: None,
            sloppy_quorum: None,
            timeout: None,
            vclock: None,
            // optional w
            dw: None,
            pw: None,
            w: None,
        }
    }

    pub fn get_bucket(&self) -> Vec<u8> {
        self.bucket.clone()
    }

    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.bucket = bucket.into();
    }

    pub fn get_content(&self) -> ObjectContent {
        self.content.clone()
    }

    pub fn set_content(&mut self, content: ObjectContent) {
        self.content = content;
    }

    pub fn get_key(&self) -> Option<Vec<u8>> {
        self.key.clone()
    }

    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.key = Some(key.into());
    }

    pub fn get_vclock(&self) -> Option<Vec<u8>> {
        self.vclock.clone()
    }

    pub fn set_vclock<T: Into<Vec<u8>>>(&mut self, vclock: T) {
        self.vclock = Some(vclock.into());
    }

    pub fn get_w(&self) -> Option<u32> {
        self.w
    }

    pub fn set_w(&mut self, w: u32) {
        self.w = Some(w);
    }

    pub fn get_dw(&self) -> Option<u32> {
        self.dw
    }

    pub fn set_dw(&mut self, dw: u32) {
        self.dw = Some(dw);
    }

    pub fn get_return_body(&self) -> Option<bool> {
        self.return_body
    }

    pub fn set_return_body(&mut self, return_body: bool) {
        self.return_body = Some(return_body);
    }

    pub fn get_pw(&self) -> Option<u32> {
        self.pw
    }

    pub fn set_pw(&mut self, pw: u32) {
        self.pw = Some(pw);
    }

    pub fn get_if_not_modified(&self) -> Option<bool> {
        self.if_not_modified
    }

    pub fn set_if_not_modified(&mut self, if_not_modified: bool) {
        self.if_not_modified = Some(if_not_modified);
    }

    pub fn get_if_none_match(&self) -> Option<bool> {
        self.if_none_match
    }

    pub fn set_if_none_match(&mut self, if_none_match: bool) {
        self.if_none_match = Some(if_none_match);
    }

    pub fn get_return_head(&self) -> Option<bool> {
        self.return_head
    }

    pub fn set_return_head(&mut self, return_head: bool) {
        self.return_head = Some(return_head);
    }

    pub fn get_timeout(&self) -> Option<u32> {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = Some(timeout);
    }

    pub fn get_asis(&self) -> Option<bool> {
        self.asis
    }

    pub fn set_asis(&mut self, asis: bool) {
        self.asis = Some(asis);
    }

    pub fn get_sloppy_quorum(&self) -> Option<bool> {
        self.sloppy_quorum
    }

    pub fn set_sloppy_quorum(&mut self, sloppy_quorum: bool) {
        self.sloppy_quorum = Some(sloppy_quorum);
    }

    pub fn get_n_val(&self) -> Option<u32> {
        self.n_val
    }

    pub fn set_n_val(&mut self, n_val: u32) {
        self.n_val = Some(n_val);
    }

    pub fn get_bucket_type(&self) -> Option<Vec<u8>> {
        self.bucket_type.clone()
    }

    pub fn set_bucket_type<T: Into<Vec<u8>>>(&mut self, bucket_type: T) {
        self.bucket_type = Some(bucket_type.into());
    }
}

impl RpbGenerator for StoreObjectReq {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr> {
        let req = store_object_req_to_rpb_put_req(&self);
        match req.write_to_bytes() {
            Ok(b) => Ok(b),
            Err(err) => Err(RiakErr::ProtobufError(err)),
        }
    }
}

/// The data used to perform a fetch object request
///
/// # Examples
///
/// ```
/// ```
#[derive(Clone, Debug)]
pub struct FetchObjectReq {
    // Required
    bucket: Vec<u8>,
    key: Vec<u8>,

    // Optional
    r: Option<u32>,
    pr: Option<u32>,
    basic_quorum: Option<bool>,
    notfound_ok: Option<bool>,
    if_modified: Option<bool>,
    head: Option<bool>,
    deletedvclock: Option<bool>,
    timeout: Option<u32>,
    sloppy_quorum: Option<bool>,
    n_val: Option<u32>,
    bucket_type: Option<Vec<u8>>,
}

impl FetchObjectReq {
    pub fn new<T: Into<Vec<u8>>>(bucket: T, key: T) -> FetchObjectReq {
        FetchObjectReq {
            bucket: bucket.into(),
            key: key.into(),
            r: None,
            pr: None,
            basic_quorum: None,
            notfound_ok: None,
            if_modified: None,
            head: None,
            deletedvclock: None,
            timeout: None,
            sloppy_quorum: None,
            n_val: None,
            bucket_type: None,
        }
    }

    pub fn get_bucket(&self) -> Vec<u8> {
        self.bucket.clone()
    }

    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.bucket = bucket.into();
    }

    pub fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }

    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.key = key.into()
    }
}

impl RpbGenerator for FetchObjectReq {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr> {
        let req = fetch_object_req_to_rpb_get_req(self);
        match req.write_to_bytes() {
            Ok(b) => Ok(b),
            Err(err) => Err(RiakErr::ProtobufError(err)),
        }
    }
}

/// Represents the response received from Riak, including all siblings of the object,
/// the vector clock, and "unchanged" if "if_modified" was set to true in the request.
#[derive(Clone, Debug)]
pub struct FetchObjectResp {
    content: Vec<ObjectContent>,
    vclock: Vec<u8>,
    unchanged: Option<bool>,
}

impl FetchObjectResp {
    pub fn new(content: &Vec<ObjectContent>, vclock: &[u8]) -> FetchObjectResp {
        FetchObjectResp {
            content: content.clone(),
            vclock: vclock.to_vec(),
            unchanged: None,
        }
    }

    pub fn get_content(&self) -> Vec<ObjectContent> {
        self.content.clone()
    }

    pub fn set_content(&mut self, content: Vec<ObjectContent>) {
        self.content = content;
    }

    pub fn get_vclock(&self) -> Vec<u8> {
        self.vclock.clone()
    }

    pub fn set_vclock(&mut self, vclock: Vec<u8>) {
        self.vclock = vclock;
    }

    pub fn get_unchanged(&self) -> Option<bool> {
        self.unchanged
    }

    pub fn set_unchanged(&mut self, unchanged: bool) {
        self.unchanged = Some(unchanged);
    }
}

/// Represents the actual value and metadata for the value being stored in Riak.
///
/// # Examples
///
/// ```
/// ```
#[derive(Clone, Debug)]
pub struct ObjectContent {
    // Required
    value: Vec<u8>,

    // Optional
    content_type: Option<Vec<u8>>,
    charset: Option<Vec<u8>>,
    content_encoding: Option<Vec<u8>>,
    vtag: Option<Vec<u8>>,
    // links: Vec<RpbLink>,
    last_mod: Option<u32>,
    last_mod_usecs: Option<u32>,
    // repeated RpbPair usermeta = 9;
    // repeated RpbPair indexes = 10;
    deleted: Option<bool>,
}

impl ObjectContent {
    pub fn new<T: Into<Vec<u8>>>(value: T) -> ObjectContent {
        ObjectContent {
            value: value.into(),
            content_type: None,
            charset: None,
            content_encoding: None,
            vtag: None,
            last_mod: None,
            last_mod_usecs: None,
            deleted: None,
        }
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn set_value<T: Into<Vec<u8>>>(&mut self, value: T) {
        self.value = value.into();
    }

    pub fn get_content_type(&self) -> Option<Vec<u8>> {
        self.content_type.clone()
    }

    pub fn set_content_type<T: Into<Vec<u8>>>(&mut self, content_type: T) {
        self.content_type = Some(content_type.into());
    }

    pub fn get_charset(&self) -> Option<Vec<u8>> {
        self.charset.clone()
    }

    pub fn set_charset<T: Into<Vec<u8>>>(&mut self, charset: T) {
        self.charset = Some(charset.into());
    }

    pub fn get_content_encoding(&self) -> Option<Vec<u8>> {
        self.content_encoding.clone()
    }

    pub fn set_content_encoding<T: Into<Vec<u8>>>(&mut self, content_encoding: T) {
        self.content_encoding = Some(content_encoding.into());
    }

    pub fn get_vtag(&self) -> Option<Vec<u8>> {
        self.vtag.clone()
    }

    pub fn set_vtag(&mut self, vtag: &[u8]) {
        self.vtag = Some(vtag.to_vec());
    }

    pub fn get_last_mod(&self) -> Option<u32> {
        self.last_mod
    }

    pub fn set_last_mod(&mut self, last_mod: u32) {
        self.last_mod = Some(last_mod);
    }

    pub fn get_last_mod_usecs(&self) -> Option<u32> {
        self.last_mod_usecs
    }

    pub fn set_last_mod_usecs(&mut self, last_mod_usecs: u32) {
        self.last_mod_usecs = Some(last_mod_usecs);
    }

    pub fn get_deleted(&self) -> Option<bool> {
        self.deleted
    }

    pub fn set_deleted(&mut self, deleted: bool) {
        self.deleted = Some(deleted);
    }
}
