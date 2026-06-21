fn run_hw_paging_smokes() -> bool {
    let mut smoke_ok = true;
    smoke_ok &= crate::foo::smoke_a();
    smoke_ok &= crate::foo::smoke_b();
    smoke_ok
}
