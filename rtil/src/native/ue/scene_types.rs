use native::ue::*;

// may be wrong
#[repr(C)]
pub struct FSceneViewStateReference {
    reference: *const FSceneViewStateInterface,
    global_list_link: TLinkedList<*const FSceneViewStateReference>,
}
