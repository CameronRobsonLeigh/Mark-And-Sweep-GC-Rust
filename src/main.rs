mod models;
use models::object::GCObject;
use models::collector::GarbageCollector;

// We don't pass mut through here because we are not modifying the variable, this is creating/reading it.
pub fn create_gc_object(gc_obj: &GCObject) -> i32
{
    if let Some(value) = gc_obj.as_any().downcast_ref::<i32>() {
        println!("The value is: {}", value);
        
        return *value;  // Dereference the reference from the heap inside the GCObject -  to return the actual i32 value
    } else {
        println!("Failed to downcast");
        return 0;
    }
}

// this paramater pass ensures exclusive mutable access to the object - not the system.
pub fn mark_gc_object(gc_obj: &mut GCObject) 
{
    gc_obj.mark(); 

    if gc_obj.is_marked() {
        println!("The GCObject is marked.");
    } else {
        println!("The GCObject is not marked.");
    }
}

pub fn unmark_gc_object(gc_obj: &mut GCObject) 
{
    gc_obj.unmark(); 

    if gc_obj.is_marked() {
        println!("The GCObject is marked.");
    } else {
        println!("The GCObject is not marked.");
    }
}

fn main() {
    // declare as a mutable binding, meaning we can reassighn or modify within its scope.
    let mut gc_obj = GCObject::new(77);

    create_gc_object(&gc_obj);
    mark_gc_object(&mut gc_obj);
    unmark_gc_object(&mut gc_obj);

}
