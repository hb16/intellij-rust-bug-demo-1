use indexmap::set::IndexSet;

fn main() {
    let index_set = &IndexSet::<String>::new();
    index_set.len();
    index_set.is_empty();
}
