mod interior_mutability;
mod reference_counting;
mod reference_cycles;
mod smart_pointer;

pub(crate) fn smart_pointer() {
    smart_pointer::heap_box();
    smart_pointer::cons_list();

    smart_pointer::deref();

    smart_pointer::drop_trait();
    smart_pointer::std_mem_drop();

    reference_counting::cons_list();

    interior_mutability::cons_list();

    reference_cycles::cons_list();
    reference_cycles::weak_reference();
}
