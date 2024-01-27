# Replicated Key-Value Store

## Description
This project aims to create a replicated key-value store based on OpenRaft and RocksDB.

## Instructions to Start a Single-Node Cluster

### Normal

Run the `ctl` script with the `start` command to start a single-node cluster:

```shell
./ctl start
```

### Release

```shell
./ctl start_release
```

### Initialize cluster after node is started

```shell
./ctl cluster_init
```

### Check cluster status

```shell
./ctl cluster_metrics
```

