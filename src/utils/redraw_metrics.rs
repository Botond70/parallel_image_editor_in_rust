use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use crate::state::app_state::RedrawKind;

const KIND_NONE: u8 = 0;
const KIND_HSV: u8 = 1;
const KIND_CROP: u8 = 2;
const KIND_RESIZE: u8 = 3;
const KIND_BLUR: u8 = 4;

static PENDING_START_NS: AtomicU64 = AtomicU64::new(0);
static PENDING_KIND: AtomicU8 = AtomicU8::new(KIND_NONE);
static PENDING_SEQ: AtomicU64 = AtomicU64::new(0);

static LAST_LOGGED_SEQ: AtomicU64 = AtomicU64::new(0);

static TOTAL_SUM_NS: AtomicU64 = AtomicU64::new(0);
static TOTAL_COUNT: AtomicU64 = AtomicU64::new(0);

static HSV_SUM_NS: AtomicU64 = AtomicU64::new(0);
static HSV_COUNT: AtomicU64 = AtomicU64::new(0);

static CROP_SUM_NS: AtomicU64 = AtomicU64::new(0);
static CROP_COUNT: AtomicU64 = AtomicU64::new(0);

static RESIZE_SUM_NS: AtomicU64 = AtomicU64::new(0);
static RESIZE_COUNT: AtomicU64 = AtomicU64::new(0);

static BLUR_SUM_NS: AtomicU64 = AtomicU64::new(0);
static BLUR_COUNT: AtomicU64 = AtomicU64::new(0);

fn kind_to_u8(kind: RedrawKind) -> u8 {
    match kind {
        RedrawKind::HSV => KIND_HSV,
        RedrawKind::Crop => KIND_CROP,
        RedrawKind::Resize => KIND_RESIZE,
        RedrawKind::Blur => KIND_BLUR,
    }
}

fn u8_to_kind(v: u8) -> Option<RedrawKind> {
    match v {
        KIND_HSV => Some(RedrawKind::HSV),
        KIND_CROP => Some(RedrawKind::Crop),
        KIND_RESIZE => Some(RedrawKind::Resize),
        KIND_BLUR => Some(RedrawKind::Blur),
        _ => None,
    }
}

pub fn mark_click_to_visible_with_start_ns(kind: RedrawKind, start_ns: u64) -> u64 {
    PENDING_START_NS.store(start_ns, Ordering::Relaxed);
    PENDING_KIND.store(kind_to_u8(kind), Ordering::Relaxed);
    PENDING_SEQ.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn mark_blur_redraw_start(start_ns: u64) -> u64 {
    PENDING_START_NS.store(start_ns, Ordering::Relaxed);
    PENDING_KIND.store(KIND_BLUR, Ordering::Relaxed);
    PENDING_SEQ.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn current_pending_seq() -> u64 {
    PENDING_SEQ.load(Ordering::Relaxed)
}

pub fn snapshot_pending_click_to_visible() -> Option<(u64, RedrawKind, u64)> {
    let start_ns = PENDING_START_NS.load(Ordering::Relaxed);
    let kind_u8 = PENDING_KIND.load(Ordering::Relaxed);
    if start_ns == 0 || kind_u8 == KIND_NONE {
        return None;
    }
    let kind = u8_to_kind(kind_u8)?;
    let seq = PENDING_SEQ.load(Ordering::Relaxed);
    Some((start_ns, kind, seq))
}

pub fn should_log_seq(seq: u64) -> bool {
    // Only the first callback for a given seq should proceed.
    LAST_LOGGED_SEQ.swap(seq, Ordering::Relaxed) != seq
}

fn record(sum: &AtomicU64, count: &AtomicU64, duration_ns: u64) -> (u64, u64) {
    let old_sum = sum.fetch_add(duration_ns, Ordering::Relaxed);
    let sum_ns = old_sum + duration_ns;
    let count_u = count.fetch_add(1, Ordering::Relaxed) + 1;
    let avg_ns = if count_u == 0 { 0 } else { sum_ns / count_u };
    (avg_ns, count_u)
}

pub fn record_visible_duration(kind: RedrawKind, duration_ns: u64) -> (u64, u64, u64, u64) {
    let (avg_total, count_total) = record(&TOTAL_SUM_NS, &TOTAL_COUNT, duration_ns);
    let (avg_kind, count_kind) = match kind {
        RedrawKind::HSV => record(&HSV_SUM_NS, &HSV_COUNT, duration_ns),
        RedrawKind::Crop => record(&CROP_SUM_NS, &CROP_COUNT, duration_ns),
        RedrawKind::Resize => record(&RESIZE_SUM_NS, &RESIZE_COUNT, duration_ns),
        RedrawKind::Blur => record(&BLUR_SUM_NS, &BLUR_COUNT, duration_ns),
    };
    (avg_total, count_total, avg_kind, count_kind)
}

