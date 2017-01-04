use errors::RiakErr;
use rpb::riak::RpbBucketProps;
use rpb::riak_yokozuna::RpbYokozunaIndex;
use yokozuna::YokozunaIndex;

pub trait BucketPropsPrivate {
    fn set_props(&mut self, RpbBucketProps);
    fn write_to_bytes<T: Into<Vec<u8>>>(&self, T, bool) -> Result<Vec<u8>, RiakErr>;
}

pub trait DeleteObjectReqPrivate {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr>;
}

pub trait YokozunaIndexPrivate {
    fn write_to_bytes(&self) -> Result<Vec<u8>, RiakErr>;
    fn new_from_rpb_yokozuna_index(RpbYokozunaIndex) -> YokozunaIndex;
}
