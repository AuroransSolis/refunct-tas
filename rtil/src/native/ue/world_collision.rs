use native::ue::*;

#[repr(C)]
pub struct AsyncTraceData {
    trace_data: TArray<TUniquePtr<TTraceThreadData<FTraceDatum>>>,
    overlap_data: TArray<TUniquePtr<TTraceThreadData<FOverlapDatum>>>,
    num_queued_trace_data: i32,
    num_queued_overlap_data: i32,
    b_async_allowed: bool,
    async_trace_completion_event: FGraphEventArray,
}

