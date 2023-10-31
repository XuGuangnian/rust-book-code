mod interior_mutability;
mod reference_counting;
mod reference_cycles;
mod smart_pointer;
pub fn smart_pointer_box() {
    smart_pointer::heap_box();
    smart_pointer::cons_list();
    smart_pointer::deref_trait();
    smart_pointer::drop_trait();
    smart_pointer::std_mem_drop();
}

pub fn smart_pointer_rc() {
    reference_counting::cons_list();
}

pub fn interior_mutability() {
    interior_mutability::cons_list();
}

pub fn reference_cycles() {
    reference_cycles::cons_list();
    reference_cycles::weak_reference();
}
