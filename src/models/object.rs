use std::any::Any; 

pub struct GCObject {
    pub marked: bool,     // determines whether the object is marked for garbage collection
    data: Box<dyn Any>, // boxed dynamic data of any type (will sit on heap) (Also called a trait object - dyn Any)
}

impl GCObject {  // implement our struct
    pub fn new<T: 'static>(value: T) -> GCObject { // fn to create new object
        GCObject {
            marked: false,
            data: Box::new(value), 
        }
    }

    // Downcasting method
    // allows us to access the data field from the heap(underlying data)(has to be original pass to Any)
    pub fn as_any(&self) -> &dyn Any {
        &*self.data
    }

    // Method to mark the object for garbage collection
    pub fn mark(&mut self) {
        self.marked = true; // Set the marked field to true
    }

    // Method to mark the object for garbage collection
    pub fn unmark(&mut self) {
        self.marked = false; // Set the marked field to true
    }

    // Method to check if the object is marked
    pub fn is_marked(&self) -> bool {
        self.marked // Return the current marked state
    }

}