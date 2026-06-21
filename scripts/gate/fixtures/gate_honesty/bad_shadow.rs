fn run_hw_paging_smokes() -> bool {
    let smoke_ok = true;
    smoke_ok &= crate::foo::smoke_a();
    let smoke_ok = crate::foo::smoke_b();
    smoke_ok
}
