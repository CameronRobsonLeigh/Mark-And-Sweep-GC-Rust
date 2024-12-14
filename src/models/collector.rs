use std::any::Any; 
use super::object::GCObject;

// define 
pub struct GarbageCollector<T> {
    objects: Vec<GCObject>,
    roots: Vec<T>,  
}

impl<T> GarbageCollector<T> {

// Create a new GarbageCollector
pub fn new() -> GarbageCollector<T> {
    GarbageCollector {
        objects: Vec::new(),
        roots:  Vec::new()
    }
}

// Add a new GCObject to the collection
pub fn add_object(&mut self, object: GCObject) {
    self.objects.push(object);
}

// Add a root reference (simulating a root object)
pub fn add_root(&mut self, object: T) {
    self.roots.push(object);
}

 // Mark phase: Start from root references and mark reachable objects
 pub fn mark(&mut self) {
    // Set all objects to unmarked
    for object in self.objects.iter_mut() {
        object.marked = false;
    }

}


 // Helper function to mark a single object and its references (if any)
 pub fn mark_object(&self, object: &mut GCObject) {
    if object.marked {
        return; // Already marked, no need to mark again
    }

    // Mark the object
    object.marked = true;

    // If the object references other objects, mark those too (pseudo-code)
    // For example, if `object` has child objects, mark them:
    // for child in object.get_children() {
    //     self.mark_object(child);
    // }
}


// Sweep phase: Remove all unmarked objects
pub fn sweep(&mut self) {
    self.objects.retain(|object| object.marked);
}

// Run the full garbage collection process: Mark and Sweep
pub fn collect_garbage(&mut self) {
    self.mark();  // Mark reachable objects - e.g. objects that have no valid references
    self.sweep(); // Sweep unreachable objects - e.g. objects that have no valid references
}


}