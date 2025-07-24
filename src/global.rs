use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChannelKey {
    Scanner,
    Assignments,
    BalancerBacklog,
    AvailableWorkers,
}

impl fmt::Display for ChannelKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
