<p  align="center">
  <picture>
    <img alt="RobustMQ Logo" src="docs/RobustMQ-logo-formal.png" width="300">
  </picture>
</p>
 <h3 align="center">
    Next generation cloud-native converged message queue.
</h3>

## What is RobustMQ?
RobustMQ is a 100% Rust-based, minimalist, highly cohesive cloud-native messaging engine. It is a typical computing, storage, scheduling separation architecture, with support for multi-protocol, clustered deployment, computing/storage layer flexibility, minimalist high cohesion architecture of the next generation of cloud-native converged message queue.

## Features
- A cloud-native messaging engine built 100% on Rust with high performance, reliability, and stability.
- Supports a variety of mainstream standard messaging protocols. Support MQTT 3.1, MQTT 5.0, AMQP, RocketMQ Remoting, Kafka Protocol, OpenMessing, JNS, SQS, WebSocket and other mainstream messaging protocols.
- Computing, storage, scheduling hierarchical architecture, computing layer, storage layer, scheduling layer have simple, fast, Serverless expansion capacity.
- Multiple virtual clusters can be created in one physical cluster.
- Flexible multilevel storage architecture can meet different service scenarios, ensuring performance and greatly reducing storage costs.
- Minimalist high cohesion architecture, no external dependent components, simple, reliable and stable architecture.

## Architecture
![Architecture Image](docs/robustmq-architecture.png)


## Introduce
- 100% Rust kernel
  
A new messaging engine kernel implemented 100% in Rust. It is hoped to build a cloud-native message engine with high performance, high reliability and high stability by using Rust language's characteristics of high performance and high security.

- Multi-protocol support
  
Support MQTT 3.1/5.0, AMQP, RocketMQ Remoting, Kafka Protocol, OpenMessing, JNS, SQS, WebSocket and other mainstream messaging protocols. It is hoped to build a message engine that can meet various scenarios to reduce the learning and operation cost of maintaining multiple message engines at the same time.

- Layered architecture
  
Typical hierarchical architecture, with independent computing, storage, scheduling layers. Each layer supports distributed cluster deployment and provides simple, fast, Serverless horizontal capacity expansion and contraction.

- Multi Tenant
  
Supports the multi-tenant feature. Multiple virtual clusters can be created on one physical cluster, and virtual clusters can be isolated from each other. From the implementation point of view, plugins support a variety of forms of stream limiting mechanisms.

- Multilevel storage
  
Designed for IDC and cloud infrastructure, the storage layer supports multiple storage media such as physical hard disks, cloud hard disks, and object storage. At the same time, performance, stability, and cost are taken into account, which is suitable for a variety of different business scenarios.

- Minimal, high cohesion
  
The architecture is refined, and there is no need to rely on external dependent components. By simplifying the complexity of the kernel architecture, it improves stability and reduces long-term learning and maintenance costs.

## Start RobustMQ MQTT
### Binary Packages
#### Download .tar.gz
```
tar -xzvf robustmq-v0.0.1-release.tar.gz
cd robustmq-v0.0.1-release
```

#### Start Placement Center
```
$ bin/robustctl placement-center start
config:/Users/bytedance/robustmq-v0.0.1-release/bin/../config/placement-center.toml
placement-center is starting....

         _____     ____    ______            _____ ________ _         _    _____
        ||    \ //    \ ||     ||||     ||||     --------||\      //|| //    \
        ||----//||      ||||____// ||     || \____   ||   || \    // ||||      ||
        ||   // ||      ||||     \||     ||      ||  ||   ||  \  //  ||||      ||
        ||_|__\ \____// ||__|__||||__|__|| __|__||  ||   ||   \//   || \___\//
                                                                               \\

placement-center started successfully.
```

#### Start MQTT Broker
```
$ bin/robustctl broker-mqtt start
config:/Users/bytedance/robustmq-v0.0.1-release/bin/../config/broker-mqtt.toml
broker-mqtt is starting....

         _____     ____    ______            _____ ________ _         _    _____
        ||    \ //    \ ||     ||||     ||||     --------||\      //|| //    \
        ||----//||      ||||____// ||     || \____   ||   || \    // ||||      ||
        ||   // ||      ||||     \||     ||      ||  ||   ||  \  //  ||||      ||
        ||_|__\ \____// ||__|__||||__|__|| __|__||  ||   ||   \//   || \___\//
                                                                               \\

broker-mqtt started successfully.
```

#### MQTT Test
MQTT functionality was tested through the MQTTX tool. MQTTX quick start: https://mqttx.app/zh/docs/get-started.

### Cargo Run

#### Run in stand-alone mode
- Run standalone by placement-center
```
cargo run --package cmd --bin placement-center -- --conf=config/placement-center.toml
```

- Run standalone by mqtt-server
```
cargo run --package cmd --bin mqtt-server -- --conf=config/mqtt-server.toml
```

### Running cluster mode

- Run cluster by mqtt-server
```
cargo run --package cmd --bin mqtt-server -- --conf=config/cluster/mqtt-server/node-1.toml
cargo run --package cmd --bin mqtt-server -- --conf=config/cluster/mqtt-server/node-2.toml
cargo run --package cmd --bin mqtt-server -- --conf=config/cluster/mqtt-server/node-3.toml
```

- Run cluster by placement-center
```
cargo run --package cmd --bin placement-center -- --conf=config/cluster/placement-center/node-1.toml
cargo run --package cmd --bin placement-center -- --conf=config/cluster/placement-center/node-2.toml
cargo run --package cmd --bin placement-center -- --conf=config/cluster/placement-center/node-3.toml
```

## License
RobustMQ uses the Apache 2.0 license to strike a balance between open contributions and allowing you to use the software however you want.

## Contributing
Please refer to contribution [guidelines](https://github.com/robustmq/robustmq) for more information.

