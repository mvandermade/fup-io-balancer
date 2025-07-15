```mermaid
graph TD
  io-router --> serial-interface
  reporter-deployment@{ shape: procs} --> io-router
```

```mermaid
sequenceDiagram
  serial-interface->>io-router: I saw a code!
  io-router->>reporter-deployment-pod-1: here is round-robin the code!
  serial-interface->>io-router: I saw another code!
  io-router->>reporter-deployment-pod-2: here is round-robin another code!
```
