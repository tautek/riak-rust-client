/// `PreflistItem` represents a Riak partition where a bucket/key combination belong, whether or
/// not that is the primary partition, and what the name of the node the partition lives on.
///
/// It can be used to optimize client requests for specific pieces keys by pointing out where the
/// key ultimately lives so you can reach it more directly and avoid passing through an
/// intermediary.
#[derive(Clone, Debug)]
pub struct PreflistItem {
    pub partition: i64,
    pub node: String,
    pub is_primary: bool,
}

impl PreflistItem {
    pub fn new(partition: i64, node: &str, is_primary: bool) -> PreflistItem {
        PreflistItem {
            partition: partition,
            node: node.to_string(),
            is_primary: is_primary,
        }
    }
}
