// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


//在存在多个引用时，编译器有时会无法自动推导生命周期，此时就需要我们手动去标注，通过为参数标注合适的生命周期来帮助编译器进行借用检查的分析

struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}


fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book: Book<'_, '_> = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
