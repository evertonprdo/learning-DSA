use lists::third::List;

fn main() {
    let list = List::new();
    assert_eq!(list.head(), None);

    println!("Before prepends");

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    println!("After prepends");

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    println!("After 1st .tail()");

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    println!("After 2st .tail()");

    let list = list.tail();
    assert_eq!(list.head(), None);

    println!("After 3st .tail()");

    let list = list.tail();
    assert_eq!(list.head(), None);

    println!("After last .tail()");
}
