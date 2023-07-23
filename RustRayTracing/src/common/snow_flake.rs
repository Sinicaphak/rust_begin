/// 实现一个雪花算法
///
/// # 示例
/// ``` rust
///  use test112::common::snow_flake::snow_flake::Snowflake;
///
///  let sf_id_worker = Snowflake::new(1, 1);
///  let id = sf_id_worker.next_id();
///  println!("{id}");
/// ```
///


pub mod snow_flake {

    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    const DEFAULT_WORKER_ID: isize = 1;
    const DEFAULT_DATA_CENTER_ID: isize = 1;

    const EPOCH: isize = 1672502400000; // 自定义开始时间 2023-01-01 00:00:00 0000
    const WORKER_ID_BITS: isize = 5;
    const DATA_CENTER_ID_BITS: isize = 5;
    const SEQUENCE_BITS: isize = 12;

    const MAX_WORKER_ID: isize = (1 << WORKER_ID_BITS) - 1;
    const MAX_DATA_CENTER_ID: isize = (1 << DATA_CENTER_ID_BITS) - 1;
    const MAX_SEQUENCE: isize = (1 << SEQUENCE_BITS) - 1;

    const WORKER_ID_SHIFT: isize = SEQUENCE_BITS;
    const DATA_CENTER_ID_SHIFT: isize = SEQUENCE_BITS + WORKER_ID_BITS;
    const TIMESTAMP_SHIFT: isize = SEQUENCE_BITS + WORKER_ID_BITS + DATA_CENTER_ID_BITS;

    pub struct Snowflake {
        worker_id: isize,
        data_center_id: isize,
        sequence: Mutex<isize>,
        last_timestamp: Mutex<isize>,
    }

    impl Snowflake {
        pub fn new(worker_id: isize, data_center_id: isize) -> Self {
            assert!(worker_id <= MAX_WORKER_ID);
            assert!(data_center_id <= MAX_DATA_CENTER_ID);
            Self {
                worker_id,
                data_center_id,
                sequence: Mutex::new(0),
                last_timestamp: Mutex::new(-1),
            }
        }
        // 使用两个默认的id
        pub fn new_default() -> Self {
            Self {
                worker_id: DEFAULT_WORKER_ID,
                data_center_id: DEFAULT_DATA_CENTER_ID,
                sequence: Mutex::new(0),
                last_timestamp: Mutex::new(-1),
            }
        }

        // 生成下一个ID
        pub fn next_id(&self) -> usize {
            let mut sequence = self.sequence.lock().unwrap();
            let mut last_timestamp = self.last_timestamp.lock().unwrap();
            let mut timestamp = Self::current_time();
            // 检查时钟是否向前移动
            if timestamp < *last_timestamp {
                panic!("Clock moved backwards");
            }
            // 如果时钟没有向前移动，则生成新的ID
            if timestamp == *last_timestamp {
                *sequence = (*sequence + 1) & MAX_SEQUENCE;
                if *sequence == 0 {
                    timestamp = Self::next_millis(*last_timestamp);
                }
            } else {
                *sequence = 0;
            }
            *last_timestamp = timestamp;
            (((timestamp - EPOCH) << TIMESTAMP_SHIFT)
                | (self.data_center_id << DATA_CENTER_ID_SHIFT)
                | (self.worker_id << WORKER_ID_SHIFT)
                | *sequence) as usize
        }

        // 获取当前时间戳
        fn current_time() -> isize {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as isize
        }

        // 如果当前时间戳小于上一个时间戳，则生成新的时间戳
        fn next_millis(last_timestamp: isize) -> isize {
            let mut timestamp = Self::current_time();
            while timestamp <= last_timestamp {
                timestamp = Self::current_time();
            }
            timestamp
        }
    }
}

