# 001 - Need a project language that is portable and works well with external devices
**Context:** The local machine of the postzegel code scanner needs a way of loadbalancing the events from the scanners. The loadbalancing is needed because the reporter software should be allowed to be run multiple times in a Kubernetes deployment.

**Decision:** Use Rust because it is highly portable, has low startup time and can interact with external devices easily. Also there are a lot of libraries available.
Alternatives: Honestly I did not check because I wanted to learn Rust. But alternatives could be: C, C++ or Go, perhaps also a GraalVM could be used?

**Consequences:** Will negatively influence velocity because Martijn has no experience in building these kinds of application using Rust. Postitive side is the learning experience 
and the small, portable and fast bootuptime of the end result.

**Participants:** Martijn van der Made

**Status:** Proposed
