/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::cmp::Ordering;
use std::collections::HashMap;

use cheetah_string::CheetahString;
use rand::seq::IteratorRandom;
use rocketmq_common::common::mix_all;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BrokerData {
    cluster: CheetahString,
    #[serde(rename = "brokerName")]
    broker_name: CheetahString,
    #[serde(rename = "brokerAddrs")]
    broker_addrs: HashMap<u64 /* broker id */, CheetahString /* broker ip */>,
    #[serde(rename = "zoneName")]
    zone_name: Option<CheetahString>,
    #[serde(rename = "enableActingMaster")]
    enable_acting_master: bool,
}

impl PartialOrd for BrokerData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BrokerData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.broker_name.cmp(&other.broker_name)
    }
}

impl BrokerData {
    pub fn new(
        cluster: CheetahString,
        broker_name: CheetahString,
        broker_addrs: HashMap<u64, CheetahString>,
        zone_name: Option<CheetahString>,
    ) -> BrokerData {
        BrokerData {
            cluster,
            broker_name,
            broker_addrs,
            zone_name,
            enable_acting_master: false,
        }
    }

    pub fn set_cluster(&mut self, cluster: CheetahString) {
        self.cluster = cluster;
    }

    pub fn set_broker_name(&mut self, broker_name: CheetahString) {
        self.broker_name = broker_name;
    }

    pub fn set_broker_addrs(&mut self, broker_addrs: HashMap<u64, CheetahString>) {
        self.broker_addrs = broker_addrs;
    }

    pub fn set_zone_name(&mut self, zone_name: Option<CheetahString>) {
        self.zone_name = zone_name;
    }

    #[inline]
    pub fn set_enable_acting_master(&mut self, enable_acting_master: bool) {
        self.enable_acting_master = enable_acting_master;
    }

    pub fn cluster(&self) -> &str {
        &self.cluster
    }

    pub fn broker_name(&self) -> &CheetahString {
        &self.broker_name
    }

    pub fn broker_addrs(&self) -> &HashMap<u64, CheetahString> {
        &self.broker_addrs
    }

    pub fn broker_addrs_mut(&mut self) -> &mut HashMap<u64, CheetahString> {
        &mut self.broker_addrs
    }

    pub fn remove_broker_by_addr(&mut self, broker_id: u64, broker_addr: &str) {
        self.broker_addrs
            .retain(|key, value| value != broker_addr || *key == broker_id);
    }

    pub fn zone_name(&self) -> &Option<CheetahString> {
        &self.zone_name
    }

    pub fn enable_acting_master(&self) -> bool {
        self.enable_acting_master
    }

    pub fn select_broker_addr(&self) -> Option<CheetahString> {
        let master_address = self.broker_addrs.get(&(mix_all::MASTER_ID)).cloned();
        if master_address.is_none() {
            return self
                .broker_addrs
                .values()
                .choose(&mut rand::thread_rng())
                .cloned();
        }
        master_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct QueueData {
    #[serde(rename = "brokerName")]
    pub broker_name: CheetahString,
    #[serde(rename = "readQueueNums")]
    pub read_queue_nums: u32,
    #[serde(rename = "writeQueueNums")]
    pub write_queue_nums: u32,
    pub perm: u32,
    #[serde(rename = "topicSysFlag")]
    pub topic_sys_flag: u32,
}

impl QueueData {
    pub fn new(
        broker_name: CheetahString,
        read_queue_nums: u32,
        write_queue_nums: u32,
        perm: u32,
        topic_sys_flag: u32,
    ) -> Self {
        Self {
            broker_name,
            read_queue_nums,
            write_queue_nums,
            perm,
            topic_sys_flag,
        }
    }

    pub fn broker_name(&self) -> &CheetahString {
        &self.broker_name
    }

    pub fn read_queue_nums(&self) -> u32 {
        self.read_queue_nums
    }

    pub fn write_queue_nums(&self) -> u32 {
        self.write_queue_nums
    }

    pub fn perm(&self) -> u32 {
        self.perm
    }

    pub fn topic_sys_flag(&self) -> u32 {
        self.topic_sys_flag
    }
}
