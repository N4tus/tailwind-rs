use super::*;

#[test]
fn test_style() {
    // w-full sm:w-auto text-lg uppercase text-gray-100 bg-purple-800 hover:bg-purple-700 focus:bg-purple-700 focus-visible:ring-4 ring-purple-400 px-6
    println!("{:#?}", TailwindInstruction::parse("not-hover:sm:text-red-200").unwrap().1);
    println!("{:#?}", TailwindInstruction::parse_list("w-full sm:w-auto").unwrap().1);
}

#[test]
fn test_group() {
    println!("{:#?}", AstGroup::parse_list("w(full sm:auto)").unwrap().1);
    println!("{:#?}", AstGroup::parse_list("not-hover:sm:text-red-200").unwrap().1);
}

#[test]
fn test_group_expand() {
    println!("{:#?}", TailwindBuilder::parse_styles("w(full sm:auto)"));
}
