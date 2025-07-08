use ic_cdk_macros::{init, pre_upgrade, post_upgrade};
use ic_stable_structures::{StableCell, memory_manager::{MemoryManager, MemoryId, VirtualMemory}, DefaultMemoryImpl};
use std::cell::RefCell;
use std::time::Duration;

// Define the type for stable memory
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Set up a memory manager and a stable cell for the timer interval
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static TIMER_INTERVAL: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            10, // default interval
        ).expect("failed to init stable cell")
    );
}

// The periodic task to run
fn periodic_task() {
    ic_cdk::println!("Timer triggered!");
}

// Set up the timer with the current interval from stable memory
fn setup_timer() {
    TIMER_INTERVAL.with(|cell| {
        let interval = cell.borrow().get();
        ic_cdk_timers::set_timer_interval(Duration::from_secs(interval), periodic_task);
    });
}

// Save timer interval to stable memory before upgrade (not needed, already in stable memory)
#[pre_upgrade]
fn pre_upgrade() {
    // No-op: TIMER_INTERVAL is already in stable memory
}

// Restore timer interval and re-establish timer after upgrade
#[post_upgrade]
fn post_upgrade() {
    setup_timer();
}

// Initialize canister and set up timer, storing interval in stable memory
#[init]
fn init(interval: u64) {
    TIMER_INTERVAL.with(|cell| {
        cell.borrow_mut().set(interval).expect("failed to set interval");
    });
    setup_timer();
}