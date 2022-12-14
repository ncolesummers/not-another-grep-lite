use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();
    

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        println!("No matches found");
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
      for (j, tag) in tags.iter().enumerate() {
        let lower_bound = tag.saturating_sub(ctx_lines);
        let upper_bound = tag + ctx_lines;

        if (i >= lower_bound) && (i >= upper_bound) {
            let line_as_string = String::from(line);
            let local_ctx = (i, line_as_string);
            ctx[j].push(local_ctx);
        }
      }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_number = i + 1;
            println!("{}: {}", line_number, line);
        }
    }
}
