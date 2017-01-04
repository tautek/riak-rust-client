// Utilities for helping in conversion to and from protobuf data

use errors::RiakErr;
use object::{ObjectContent, StoreObjectReq, FetchObjectReq, FetchObjectResp};
use rpb::riak_kv::{RpbContent, RpbPutReq, RpbGetReq, RpbGetResp};

// `RpbGeneratorID` is a trait for structs that can be converted to protobuf bytes, with some
// identifying piece of information needed to add to the protobuf conversion.
pub trait RpbGeneratorID {
    fn write_to_bytes(&self, &str) -> Result<Vec<u8>, RiakErr>;
}

// `RpbGenerator` is a trait for structs that can be converted to protobuf bytes requiring only the
// data that they already contain.
pub trait RpbGenerator {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr>;
}

// Renders a `RpbPutReq` given a `StoreObjectReq`.
pub fn store_object_req_to_rpb_put_req(store_object_req: &StoreObjectReq) -> RpbPutReq {
    let mut rpb_put_req = RpbPutReq::new();

    let bucket_name = store_object_req.get_bucket().clone();
    rpb_put_req.set_bucket(bucket_name);

    let content = object_content_to_rpb_content(&store_object_req.get_content());
    rpb_put_req.set_content(content);

    match store_object_req.get_key() {
        Some(v) => rpb_put_req.set_key(v.clone()),
        None => (),
    };
    match store_object_req.get_vclock() {
        Some(v) => rpb_put_req.set_vclock(v.clone()),
        None => (),
    };
    match store_object_req.get_w() {
        Some(v) => rpb_put_req.set_w(v),
        None => (),
    };
    match store_object_req.get_dw() {
        Some(v) => rpb_put_req.set_dw(v),
        None => (),
    };
    match store_object_req.get_return_body() {
        Some(v) => rpb_put_req.set_return_body(v),
        None => (),
    };
    match store_object_req.get_pw() {
        Some(v) => rpb_put_req.set_pw(v),
        None => (),
    };
    match store_object_req.get_if_not_modified() {
        Some(v) => rpb_put_req.set_if_not_modified(v),
        None => (),
    };
    match store_object_req.get_if_none_match() {
        Some(v) => rpb_put_req.set_if_none_match(v),
        None => (),
    };
    match store_object_req.get_return_head() {
        Some(v) => rpb_put_req.set_return_head(v),
        None => (),
    };
    match store_object_req.get_timeout() {
        Some(v) => rpb_put_req.set_timeout(v),
        None => (),
    };
    match store_object_req.get_asis() {
        Some(v) => rpb_put_req.set_asis(v),
        None => (),
    };
    match store_object_req.get_sloppy_quorum() {
        Some(v) => rpb_put_req.set_sloppy_quorum(v),
        None => (),
    };
    match store_object_req.get_n_val() {
        Some(v) => rpb_put_req.set_n_val(v),
        None => (),
    };
    match store_object_req.get_bucket_type() {
        Some(v) => rpb_put_req.set_field_type(v.clone()),
        None => (),
    };

    rpb_put_req
}

// Renders a `RpbContent` given an `ObjectContent`.
pub fn object_content_to_rpb_content(object_content: &ObjectContent) -> RpbContent {
    let mut rpb_content = RpbContent::new();

    rpb_content.set_value(object_content.get_value());

    match object_content.get_content_type() {
        Some(v) => rpb_content.set_content_type(v.clone()),
        None => (),
    };
    match object_content.get_charset() {
        Some(v) => rpb_content.set_charset(v.clone()),
        None => (),
    };
    match object_content.get_content_encoding() {
        Some(v) => rpb_content.set_content_encoding(v.clone()),
        None => (),
    };
    match object_content.get_vtag() {
        Some(v) => rpb_content.set_vtag(v.clone()),
        None => (),
    };
    match object_content.get_last_mod() {
        Some(v) => rpb_content.set_last_mod(v),
        None => (),
    };
    match object_content.get_last_mod_usecs() {
        Some(v) => rpb_content.set_last_mod_usecs(v),
        None => (),
    };
    match object_content.get_deleted() {
        Some(v) => rpb_content.set_deleted(v),
        None => (),
    };

    rpb_content
}

// Renders a `ObjectContent` given an `RpbContent`.
pub fn rpb_content_to_object_content(rpb_content: &mut RpbContent) -> ObjectContent {
    let value = rpb_content.take_value();
    let mut object_content = ObjectContent::new(value);

    if rpb_content.has_content_type() {
        let content_type = rpb_content.get_content_type().clone();
        object_content.set_content_type(content_type);
    }

    if rpb_content.has_charset() {
        let charset = rpb_content.get_charset().clone();
        object_content.set_charset(charset);
    }

    if rpb_content.has_content_encoding() {
        let content_encoding = rpb_content.get_content_encoding().clone();
        object_content.set_content_encoding(content_encoding);
    }

    object_content
}

// Renders a `RpbGetReq` from a `FetchObjectReq`.
pub fn fetch_object_req_to_rpb_get_req(fetch_object_req: &FetchObjectReq) -> RpbGetReq {
    let mut rpb_get_req = RpbGetReq::new();

    rpb_get_req.set_bucket(fetch_object_req.get_bucket().clone());
    rpb_get_req.set_key(fetch_object_req.get_key().clone());

    // TODO

    rpb_get_req
}

// Renders a `FetchObjectResp` from `RpbGetResp`
pub fn rpb_get_resp_to_fetch_object_resp(rpb_get_resp: &mut RpbGetResp) -> FetchObjectResp {
    let mut siblings: Vec<ObjectContent> = Vec::new();
    let rpb_content = rpb_get_resp.take_content();

    for content in rpb_content.iter() {
        let mut c = content.clone();
        let converted = rpb_content_to_object_content(&mut c);
        siblings.push(converted);
    }

    let mut fetch_object_resp = FetchObjectResp::new(&siblings, rpb_get_resp.get_vclock());

    if rpb_get_resp.has_unchanged() {
        fetch_object_resp.set_unchanged(rpb_get_resp.get_unchanged());
    }

    fetch_object_resp
}
