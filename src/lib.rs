pub fn mutate_inplace(input: &mut String) {
    input.insert_str(0, "<i>");
    input.push_str("</i>");
}

pub fn fmt(input: String) -> String {
    format!("<i>{}</i>", input)
}

pub fn with_cap(input: String) -> String {
    let mut new = String::with_capacity(input.len() + 7);
    new.push_str("<i>");
    new.push_str(&input);
    new.push_str("</i>");

    new
}
