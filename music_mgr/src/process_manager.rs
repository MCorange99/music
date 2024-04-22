use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use tokio::{process::Command, sync::{Mutex, RwLock}};



#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Proc {
    msg: String,
    finished: bool
}

lazy_static::lazy_static!(
    static ref PROCESSES: Mutex<RwLock<HashMap<usize, Proc>>> = Mutex::new(RwLock::new(HashMap::new()));
);

static PROC_INC: AtomicUsize = AtomicUsize::new(0);



pub async fn add_proc(mut cmd: Command, msg: String) -> anyhow::Result<()> {
    let mut proc = cmd.spawn()?;
    let id = PROC_INC.fetch_add(1, Ordering::AcqRel);
    
    tokio::spawn(async move {
        let id = id;
        proc.wait().await
            .expect("child process encountered an error");
        PROCESSES.lock().await.write().await.get_mut(&id).unwrap().finished = true;
    });
    
    PROCESSES.lock().await.write().await.insert(id, Proc {
        finished: false,
        msg,
    });

    Ok(())
}

/// Waits for processes to finish untill the proc count is lower or equal to `max`
pub async fn wait_for_procs_untill(max: usize) -> anyhow::Result<usize> {
    // NOTE: This looks really fucked because i dont want to deadlock the processes so i lock PROCESSES for as little as possible
    // NOTE: So its also kinda really slow
    let mut finish_count = 0;
    loop {
        {
            if PROCESSES.lock().await.read().await.len() <= max {
                return Ok(finish_count);
            }
        }

        let procs = {
            PROCESSES.lock().await.read().await.clone()
        };

        for (idx, proc) in procs {
            if proc.finished {
                {
                    PROCESSES.lock().await.write().await.remove(&idx);
                }
                log::info!("{}", proc.msg);
                finish_count += 1;
            }
        }
    }
}